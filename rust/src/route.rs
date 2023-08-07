use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::*,
    mongo::MONGO,
    pg::PG,
};

pub fn create_router(pg: PG, mongo: MONGO) -> Router {

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .nest("/api/pg",Router::new()
            .route(
                "/",
                post(create_customer_handler).get(list_customer_handler)
            )
            .route(
                "/:name",
                get(get_customer_handler).delete(delete_customer_handler).patch(update_customer_handler)
            )
            .with_state(pg)
        )
        .nest("/api/mongo",Router::new()
            .route(
                "/",
                post(create_order_handler).get(list_order_handler)
            )
            .route(
                "/:id",
                get(get_order_handler).patch(update_order_handler).delete(delete_order_handler)
            )
            .with_state(mongo)
        )
        
}
