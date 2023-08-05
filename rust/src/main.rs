mod db;
mod error;
mod handler;
mod model;
mod response;
mod schema;
mod pg;

use db::DB;
use pg::PG;
use dotenv::dotenv;
use schema::FilterOptions;
use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    // todo remove this
    dotenv().ok();
    let db = DB::init().await?;
    let pg = PG::init().await?;

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:8000"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let note_router = 
        warp::path!("api" / "notes");
    let note_router_id = 
        warp::path!("api" / "notes" / String);
    let health_checker = 
        warp::path!("api" / "healthchecker").and(warp::get())
                                            .and_then(handler::health_checker_handler);

    let note_routes = note_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_note_handler)
        .or(note_router
            .and(warp::get())
            .and(warp::query::<FilterOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::notes_list_handler));

    let note_routes_id = note_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_note_handler)
        .or(note_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_note_handler))
        .or(note_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_note_handler));

    let pg_router = 
        warp::path!("api" / "pg");
    let pg_router_id = 
        warp::path!("api" / "pg" / String);

    let pg_routes = pg_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pg(pg.clone()))
        .and_then(handler::create_customer_handler)
        .or(pg_router
            .and(warp::get())
            .and(warp::query::<FilterOptions>())
            .and(with_pg(pg.clone()))
            .and_then(handler::list_customer_handler));


    let pg_routes_id = pg_router_id
        .and(warp::get())
        .and(with_pg(pg.clone()))
        .and_then(handler::get_customer_handler);

    let routes = note_routes
        .or(note_routes_id)
        .or(health_checker)
        .or(pg_routes)
        .or(pg_routes_id)
        .with(cors)
        .recover(error::handle_rejection)
        .with(warp::log("api"));

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_pg(db: PG) -> impl Filter<Extract = (PG,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
