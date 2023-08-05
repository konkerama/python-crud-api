use crate::response::CustResponse;
use crate::{
    error::Error::*, model::CustModel, schema::CreateCustSchema, Result,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct PG {
    pub pool: Pool<Postgres>,
}

impl PG {
    pub async fn init() -> Result<Self> { 
        let database_url = "postgresql://postgres:postgres@postgres:5432/postgres";
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        let _query_result = sqlx::query!(
            "CREATE TABLE IF NOT EXISTS customer (customer_name varchar,customer_surname varchar)"
            )
            .execute(&pool.clone())
            .await
            .map_err(SqlxError)?;

        Ok(Self {
            pool,
        })
    }

    pub async fn create_customer(&self, body: &CreateCustSchema) -> Result<Option<CustResponse>> {
        let name = body.customer_name.to_owned();
        let surname = body.customer_surname.to_owned();

        let query_result = sqlx::query_as!(
            CustModel,
            "INSERT INTO customer (customer_name,customer_surname) VALUES ($1, $2) RETURNING *",
            name,
            surname,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(SqlxError)?;

        let cust_response = CustResponse {
            name: query_result.customer_name.unwrap_or("john doe".to_string()),
            status: "success".to_string(),
        };

        Ok(Some(cust_response))
    }

    // fn doc_to_note(&self, note: &NoteModel) -> Result<NoteResponse> {
    //     let note_response = NoteResponse {
    //         id: note.id.to_hex(),
    //         title: note.title.to_owned(),
    //         content: note.content.to_owned(),
    //         category: note.category.to_owned().unwrap(),
    //         published: note.published.unwrap(),
    //         createdAt: note.createdAt,
    //         updatedAt: note.updatedAt,
    //     };

    //     Ok(note_response)
    // }
}
