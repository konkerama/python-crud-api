// mod db;
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
use axum::{middleware, Json, Router};
use axum::response::{Html, IntoResponse, Response};
use serde_json::json;


// use db::DB;
use pg::PG;
use route::create_router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=trace");
    }
    let pg = PG::init().await.unwrap();


    // todo remove this
    // dotenv().ok();


    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(pg.clone())
                .layer(cors)
                .layer(middleware::map_response(main_response_mapper))
                .layer(TraceLayer::new_for_http());

    let app = app.fallback(handler::handler_404);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

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

				println!("    ->> client_error_body: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	// let _ =
	// 	log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	// println!();
	error_response.unwrap_or(res)
}


    // let cors = warp::cors()
    //     .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_origins(vec!["http://localhost:8000"])
    //     .allow_headers(vec!["content-type"])
    //     .allow_credentials(true);

//     let note_router = 
//         warp::path!("api" / "notes");
//     let note_router_id = 
//         warp::path!("api" / "notes" / String);
//     let health_checker = 
//         warp::path!("api" / "healthchecker").and(warp::get())
//                                             .and_then(handler::health_checker_handler);

//     let note_routes = note_router
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(with_db(db.clone()))
//         .and_then(handler::create_note_handler)
//         .or(note_router
//             .and(warp::get())
//             .and(warp::query::<FilterOptions>())
//             .and(with_db(db.clone()))
//             .and_then(handler::notes_list_handler));

//     let note_routes_id = note_router_id
//         .and(warp::patch())
//         .and(warp::body::json())
//         .and(with_db(db.clone()))
//         .and_then(handler::edit_note_handler)
//         .or(note_router_id
//             .and(warp::get())
//             .and(with_db(db.clone()))
//             .and_then(handler::get_note_handler))
//         .or(note_router_id
//             .and(warp::delete())
//             .and(with_db(db.clone()))
//             .and_then(handler::delete_note_handler));

//     let pg_router = 
//         warp::path!("api" / "pg");
//     let pg_router_id = 
//         warp::path!("api" / "pg" / String);

//     let pg_routes = pg_router
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(with_pg(pg.clone()))
//         .and_then(handler::create_customer_handler)
//         .or(pg_router
//             .and(warp::get())
//             .and(warp::query::<FilterOptions>())
//             .and(with_pg(pg.clone()))
//             .and_then(handler::list_customer_handler));


//     let pg_routes_id = pg_router_id
//         .and(warp::get())
//         .and(with_pg(pg.clone()))
//         .and_then(handler::get_customer_handler);

//     let routes = note_routes
//         .or(note_routes_id)
//         .or(health_checker)
//         .or(pg_routes)
//         .or(pg_routes_id)
//         .with(cors)
//         .recover(error::handle_rejection)
//         .with(warp::log("api"));

//     println!("🚀 Server started successfully");
//     warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
//     Ok(())
// }

// fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
//     warp::any().map(move || db.clone())
// }

// fn with_pg(db: PG) -> impl Filter<Extract = (PG,), Error = Infallible> + Clone {
//     warp::any().map(move || db.clone())
// }
