use crate::{
    db::DB,
    pg::PgDB,
    response::GenericResponse,
    schema::UpdateNoteSchema,
    schema::{CreateNoteSchema, FilterOptions, CreateCustomerSchema},
    WebResult,
};
use warp::{http::StatusCode, reject, reply::json, reply::with_status, Reply};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
type PgPool = Pool<ConnectionManager<PgConnection>>;


pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn notes_list_handler(opts: FilterOptions, db: DB) -> WebResult<impl Reply> {
    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    let result_json = db
        .fetch_notes(limit, page)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json(&result_json))
}

pub async fn create_note_handler(body: CreateNoteSchema, db: DB) -> WebResult<impl Reply> {
    let note = db.create_note(&body).await.map_err(|e| reject::custom(e))?;

    Ok(with_status(json(&note), StatusCode::CREATED))
}

pub async fn get_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let note = db.get_note(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    if note.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&note), StatusCode::OK))
}

pub async fn edit_note_handler(
    id: String,
    body: UpdateNoteSchema,
    db: DB,
) -> WebResult<impl Reply> {
    let note = db
        .edit_note(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    if note.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&note), StatusCode::OK))
}

pub async fn delete_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let result = db.delete_note(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    if result.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&""), StatusCode::NO_CONTENT))
}

// pub async fn customer_list_handler(opts: FilterOptions, db: PgDB) -> WebResult<impl Reply> {
//     let limit = opts.limit.unwrap_or(10) as i64;
//     let page = opts.page.unwrap_or(1) as i64;

//     let result_json = db
//         .fetch_customer(limit, page)
//         .await
//         .map_err(|e| reject::custom(e))?;

//     Ok(json(&result_json))
// }

pub async fn create_customer_handler(body: CreateCustomerSchema, mut db: PgPool) -> WebResult<impl Reply> {
    let note = db.create_customer(&body).await.map_err(|e| reject::custom(e))?;

    Ok(with_status(json(&note), StatusCode::CREATED))
}