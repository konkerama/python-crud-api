# syntax=docker/dockerfile:1

ARG PYTHON_VERSION=3.14
ARG UV_VERSION=0.5.14

FROM ghcr.io/astral-sh/uv:${UV_VERSION} AS uv
FROM python:${PYTHON_VERSION}-slim AS base
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

# Install Python dependencies (locked) into a dedicated venv using uv.
FROM base AS deps
ENV UV_PROJECT_ENVIRONMENT=/opt/venv \
    PATH="/opt/venv/bin:/root/.local/bin:${PATH}"

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        build-essential \
        python3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=uv /uv /usr/local/bin/uv

WORKDIR /srv/flask_app
COPY pyproject.toml uv.lock ./

# --no-install-project because this repo runs from /srv/flask_app/app and isn't packaged.
RUN uv sync --frozen --no-dev --no-install-project

# Runtime image: nginx + the prebuilt venv + your app code.
FROM base AS runtime
ENV PATH="/opt/venv/bin:${PATH}"

RUN apt-get update \
    && apt-get install -y --no-install-recommends nginx libexpat1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /srv/flask_app/server
COPY --from=deps /opt/venv /opt/venv
COPY . /srv/flask_app
COPY ./server/nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080
RUN chmod +x ./start.sh
CMD ["./start.sh"]