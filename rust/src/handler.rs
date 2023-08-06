use crate::{
    // db::DB,
    pg::PG,
    response::{GenericResponse,CustomerResponse, CustomerListResponse},
    schema::UpdateNoteSchema,
    schema::{CreateNoteSchema, CreateCustomerSchema, FilterOptions},
};
use crate::{Error, Result};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn health_checker_handler() -> Result<impl IntoResponse> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";
    println!("{}",MESSAGE);
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok((StatusCode::OK, Json(serde_json::json!(response_json))))
}

// pub async fn notes_list_handler(opts: FilterOptions, db: DB) -> WebResult<impl Reply> {
//     let limit = opts.limit.unwrap_or(10) as i64;
//     let page = opts.page.unwrap_or(1) as i64;

//     let result_json = db
//         .fetch_notes(limit, page)
//         .await
//         .map_err(|e| reject::custom(e))?;

//     Ok(json(&result_json))
// }

// pub async fn create_note_handler(body: CreateNoteSchema, db: DB) -> WebResult<impl Reply> {
//     let note = db.create_note(&body).await.map_err(|e| reject::custom(e))?;

//     Ok(with_status(json(&note), StatusCode::CREATED))
// }

// pub async fn get_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
//     let note = db.get_note(&id).await.map_err(|e| reject::custom(e))?;

//     let error_response = GenericResponse {
//         status: "fail".to_string(),
//         message: format!("Note with ID: {} not found", id),
//     };

//     if note.is_none() {
//         return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
//     }

//     Ok(with_status(json(&note), StatusCode::OK))
// }

// pub async fn edit_note_handler(
//     id: String,
//     body: UpdateNoteSchema,
//     db: DB,
// ) -> WebResult<impl Reply> {
//     let note = db
//         .edit_note(&id, &body)
//         .await
//         .map_err(|e| reject::custom(e))?;

//     let error_response = GenericResponse {
//         status: "fail".to_string(),
//         message: format!("Note with ID: {} not found", id),
//     };

//     if note.is_none() {
//         return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
//     }

//     Ok(with_status(json(&note), StatusCode::OK))
// }

// pub async fn delete_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
//     let result = db.delete_note(&id).await.map_err(|e| reject::custom(e))?;

//     let error_response = GenericResponse {
//         status: "fail".to_string(),
//         message: format!("Note with ID: {} not found", id),
//     };

//     if result.is_none() {
//         return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
//     }

//     Ok(with_status(json(&""), StatusCode::NO_CONTENT))
// }

// POST /api/pg
#[axum_macros::debug_handler]
pub async fn create_customer_handler(
    State(db): State<PG>,
    Json(mut body): Json<CreateCustomerSchema>,
) -> Result<impl IntoResponse> {


    let result = db.create_customer(&body)
        .await?;


    Ok((StatusCode::CREATED, Json(result)))

}

pub async fn handler_404() -> impl IntoResponse {
    
    (StatusCode::FORBIDDEN, "nothing to see here")
}

// GET /api/pg
pub async fn list_customer_handler(
    opts: Option<Query<FilterOptions>>, 
    State(db): State<PG>,
) -> Result<Json<CustomerListResponse>> {

    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10) as i64;
    let offset = opts.page.unwrap_or(1) as i64;
    let result = 
        db.list_customers(limit, offset)
        .await?;

    Ok(Json(result.unwrap()))
}

// // GET /api/pg/<customer-name>
// pub async fn get_customer_handler(id: String, db: PG) -> WebResult<impl Reply> {
//     let result = db.get_customer(&id).await.map_err(|e| reject::custom(e))?;

//     Ok(with_status(json(&result), StatusCode::OK))

// }