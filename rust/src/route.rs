use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::*,
    // db::DB,
    pg::PG,
};

pub fn create_router(pg: PG) -> Router {

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/pg",
            post(create_customer_handler).get(list_customer_handler)
        )
        .route(
            "/api/pg/:name",
            get(get_customer_handler).delete(delete_customer_handler).patch(update_customer_handler)
        )
        .with_state(pg)
}
