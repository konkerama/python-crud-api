from __future__ import annotations

import os
import sys
from typing import Any

from loguru import logger


def _hex_trace_id(trace_id: int) -> str:
    # OTel trace_id is 16 bytes (32 hex chars)
    return f"{trace_id:032x}"


def _hex_span_id(span_id: int) -> str:
    # OTel span_id is 8 bytes (16 hex chars)
    return f"{span_id:016x}"


def _trace_url(trace_id_hex: str) -> str | None:
    """Build a clickable trace URL from an environment template.

    Set `OTEL_TRACE_URL_TEMPLATE` to something like:
    - Jaeger: http://localhost:16686/trace/{trace_id}

    Only `{trace_id}` is supported.
    """

    template = os.getenv("OTEL_TRACE_URL_TEMPLATE", "").strip()
    if not template:
        return None
    return template.replace("{trace_id}", trace_id_hex)


def configure_logging(*, enabled: bool) -> None:
    """Configure Loguru to include trace/span correlation fields.

    This is safe to call whether telemetry is enabled or not.
    When OpenTelemetry isn't present or no span is active, fields are '-'.
    """

    def _patch(record: dict) -> None:
        record.setdefault("extra", {})
        record["extra"].setdefault("trace_id", "-")
        record["extra"].setdefault("span_id", "-")
        record["extra"].setdefault("trace_url", "-")

        if not enabled:
            return

        try:
            from opentelemetry import trace
        except ModuleNotFoundError:
            return

        span = trace.get_current_span()
        if span is None:
            return

        ctx = span.get_span_context()
        if ctx is None or not getattr(ctx, "is_valid", False):
            return

        trace_id_hex = _hex_trace_id(ctx.trace_id)
        record["extra"]["trace_id"] = trace_id_hex
        record["extra"]["span_id"] = _hex_span_id(ctx.span_id)
        record["extra"]["trace_url"] = _trace_url(trace_id_hex) or "-"

    # Make log output deterministic and always include correlation fields.
    # This only affects loguru logs (not stdlib logging/uvicorn logs).
    logger.remove()
    logger.configure(patcher=_patch)
    logger.add(
        sys.stderr,
        level=os.getenv("LOG_LEVEL", "INFO"),
        backtrace=False,
        diagnose=False,
        enqueue=True,
        format=(
            "{time:YYYY-MM-DD HH:mm:ss.SSS} | {level: <8} | "
            "trace_id={extra[trace_id]} span_id={extra[span_id]} trace_url={extra[trace_url]} | "
            "{name}:{function}:{line} - {message}{exception}"
        ),
    )


def _parse_sampler() -> Any:
    sampler = os.getenv("OTEL_TRACES_SAMPLER", "always_on").strip().lower()
    sampler_arg = os.getenv("OTEL_TRACES_SAMPLER_ARG", "").strip()

    from opentelemetry.sdk.trace.sampling import (
        ALWAYS_OFF,
        ALWAYS_ON,
        ParentBased,
        TraceIdRatioBased,
    )

    if sampler in {"always_on", "parentbased_always_on"}:
        return ParentBased(ALWAYS_ON)
    if sampler in {"always_off", "parentbased_always_off"}:
        return ParentBased(ALWAYS_OFF)
    if sampler in {"traceidratio", "parentbased_traceidratio"}:
        try:
            ratio = float(sampler_arg) if sampler_arg else 1.0
        except ValueError:
            ratio = 1.0
        ratio = max(0.0, min(1.0, ratio))
        return ParentBased(TraceIdRatioBased(ratio))

    logger.warning(f"Unknown OTEL_TRACES_SAMPLER='{sampler}', defaulting to always_on")
    return ParentBased(ALWAYS_ON)


def configure_telemetry(*, app: Any, engine: Any | None, enabled: bool) -> None:
    """Configure OpenTelemetry auto-instrumentation.

    When disabled, this function is a no-op: no providers/exporters are created and
    no instrumentation is enabled.
    """

    if not enabled:
        return

    try:
        from opentelemetry import trace
        from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import (
            OTLPSpanExporter,
        )
        from opentelemetry.sdk.resources import Resource
        from opentelemetry.sdk.trace import TracerProvider
        from opentelemetry.sdk.trace.export import BatchSpanProcessor

        from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
        from opentelemetry.instrumentation.requests import RequestsInstrumentor
        from opentelemetry.instrumentation.sqlalchemy import SQLAlchemyInstrumentor
        from opentelemetry.instrumentation.pymongo import PymongoInstrumentor
    except ModuleNotFoundError as exc:
        logger.warning(
            "Telemetry is enabled but OpenTelemetry deps are missing. "
            f"Install required packages; missing: {exc.name}"
        )
        return

    service_name = (
        os.getenv("OTEL_SERVICE_NAME") or os.getenv("SERVICE_NAME") or "python-crud-api"
    )

    # Resource.create() also reads OTEL_RESOURCE_ATTRIBUTES; we add a few sane defaults.
    resource = Resource.create(
        {
            "service.name": service_name,
            "deployment.environment": os.getenv("ENV", "dev"),
            "application.name": service_name,
        }
    )

    provider = TracerProvider(resource=resource, sampler=_parse_sampler())
    provider.add_span_processor(BatchSpanProcessor(OTLPSpanExporter()))
    trace.set_tracer_provider(provider)

    # Incoming HTTP
    FastAPIInstrumentor.instrument_app(app)

    # Outgoing HTTP
    RequestsInstrumentor().instrument()

    # SQLAlchemy (SQLModel uses SQLAlchemy under the hood)
    if engine is not None:
        SQLAlchemyInstrumentor().instrument(engine=engine)

    # MongoDB
    PymongoInstrumentor().instrument()

    # Ensure spans are flushed on shutdown.
    try:
        app.add_event_handler("shutdown", provider.shutdown)
    except Exception:
        # If app isn't a FastAPI instance for some reason, just skip.
        pass

    logger.info(
        "OpenTelemetry enabled (auto-instrumentation): "
        "fastapi, requests, sqlalchemy, pymongo"
    )
