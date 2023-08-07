mod mongo;
mod error;
mod handler;
mod model;
mod response;
mod schema;
mod pg;
mod route;

pub use self::error::{Error, Result};


use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::{middleware, Json};
use axum::response::{IntoResponse, Response};
use serde_json::json;

use mongo::MONGO;
use pg::PG;
use route::create_router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing::level_filters::LevelFilter;
use tracing::Level;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "app=info,tower_http=trace");
    }
    let pg = PG::init().await.unwrap();
    let mongo = MONGO::init().await.unwrap();

    // todo remove this
    // dotenv().ok();

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");


    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(pg.clone(), mongo.clone())
                .layer(cors)
                .layer(middleware::map_response(main_response_mapper))
                .layer(TraceLayer::new_for_http());

    let app = app.fallback(handler::handler_404);

    tracing::info!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[allow(unused_variables)]
async fn main_response_mapper(
	req_method: Method,
	res: Response,
) -> Response {
	tracing::info!("->> {:<12} - main_response_mapper", "RES_MAPPER");

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
					}
				});

				tracing::error!("    ->> client_error_body: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log the server log line.
	// let client_error = client_status_error.unzip().1;
    // tracing::error!("Method: {:?}, client error: {:?}", req_method, client_error);
	// TODO: Need to hander if log_request fail (but should not fail request)

	error_response.unwrap_or(res)
}