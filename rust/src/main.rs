mod db;
mod pg;
mod error;
mod handler;
mod model;
mod response;
mod schema;
mod data_access;


use db::DB;
use crate::data_access::DBAccessManager;
use dotenv::dotenv;
use schema::FilterOptions;
use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection};
// use sqlx::postgres::PgPool;


use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};


type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    // todo remove this
    dotenv().ok();
    let db = DB::init().await?;
    // let pg_db = PgDB::init().await?;
    let pg_username: String = 
        std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
    let pg_passwd: String = 
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
    let pg_db: String = 
        std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set.");
    let pg_domain: String = 
        std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set.");
    let pg_uri = 
        format!("postgresql://{}:{}@{}:5243/{}", pg_username, pg_passwd,pg_domain, pg_db);

    let pg_pool = pg_pool(pg_uri.as_str());



    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let note_router = 
        warp::path!("api" / "notes");
    let note_router_id = 
        warp::path!("api" / "notes" / String);
    let customer_router = 
        warp::path!("api" / "customer");
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

    let customer_routes = customer_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pg_db(pg_pool.clone()))
        .and_then(handler::create_customer_handler);
    //     // .or(customer_router
    //     //     .and(warp::get())
    //     //     .and(warp::query::<FilterOptions>())
    //     //     .and(with_db(db.clone()))
    //     //     .and_then(handler::customer_list_handler));

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

    let routes = note_routes
        .with(warp::log("api"))
        .or(note_routes_id)
        .or(health_checker)
        .with(cors)
        .recover(error::handle_rejection);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_db_access_manager(pool: PgPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {  match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(reject::custom(
                AppError::new(format!("Error getting connection from pool: {}", err.to_string()).as_str(), ErrorType::Internal))
            ),
        }})
}