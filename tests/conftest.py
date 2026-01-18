import os
import sys

import pytest


@pytest.fixture(scope="session")
def main_module(tmp_path_factory):
    """Import `app.main` once with test-safe env vars.

    `app.main` reads required env vars and defines SQLModel tables at import time.
    Reloading would re-define tables on the same global SQLModel metadata, so we
    avoid reloads and instead perform a single clean import for the whole session.
    """

    os.environ.setdefault("ME_CONFIG_MONGODB_ADMINUSERNAME", "test")
    os.environ.setdefault("ME_CONFIG_MONGODB_ADMINPASSWORD", "test")
    os.environ.setdefault("ME_CONFIG_MONGODB_SERVER", "test")

    os.environ.setdefault("POSTGRES_USER", "test")
    os.environ.setdefault("POSTGRES_PASSWORD", "test")
    os.environ.setdefault("POSTGRES_DB", "test")

    db_dir = tmp_path_factory.mktemp("sqlite")
    sqlite_path = db_dir / "test.db"
    os.environ["POSTGRES_URL"] = f"sqlite:///{sqlite_path}"

    os.environ.setdefault("ENABLE_TELEMETRY", "false")

    sys.modules.pop("app.main", None)
    import app.main  # noqa: E402

    app.main.init_db()
    return app.main
