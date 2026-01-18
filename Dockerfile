FROM python:3.14-slim

# Install uv.
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

WORKDIR /app

# Copy only what we need for dependency installation (better cache reuse).
COPY pyproject.toml uv.lock /app/
COPY app /app/app

# Install runtime dependencies only.
RUN uv sync --frozen --no-cache --no-dev

# Run the application.
CMD ["/app/.venv/bin/fastapi", "run", "app/main.py", "--port", "8080", "--host", "0.0.0.0"]