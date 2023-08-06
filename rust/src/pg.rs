use crate::response::{CustomerResponse, SingleCustomerResponse, CustomerListResponse};
use crate::{
    model::CustomerModel, schema::CreateCustomerSchema,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct PG {
    pub pool: Pool<Postgres>,
}

impl PG {
    pub async fn init() -> Result<Self> { 
        let database_url = "postgresql://postgres:postgres@localhost:5432/postgres";
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

        Ok(Self {
            pool,
        })
    }

    pub async fn create_customer(&self, body: &CreateCustomerSchema) -> Result<SingleCustomerResponse> {
        let name = body.customer_name.to_owned();
        let surname = body.customer_surname.to_owned();

        let query_result = sqlx::query_as!(
            CustomerModel,
            "INSERT INTO customer (customer_name,customer_surname) VALUES ($1, $2) RETURNING *",
            name,
            surname,
        )
        .fetch_one(&self.pool)
        .await.unwrap();

        let customer_response = SingleCustomerResponse {
            name: query_result.customer_name.unwrap_or("john doe".to_string()),
            surname: query_result.customer_surname.unwrap_or("doe".to_string()),
            status: "success".to_string(),
        };

        Ok(customer_response)
    }

    pub async fn list_customers(&self, limit: i64, offset: i64) -> Result<Option<CustomerListResponse>> {

        let query_result = sqlx::query_as!(
            CustomerModel,
            "SELECT * FROM customer ORDER by customer_name LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e|{Error::LoginFail})?;

        println!("{:?}", query_result);

        let mut json_result: Vec<CustomerResponse> = Vec::new();
        for customer in query_result {
            json_result.push(self.model_to_result(&customer).unwrap());
        }

        let customer_response = CustomerListResponse {
            status: "success".to_string(),
            data: json_result
        };

        Ok(Some(customer_response))
    }

    // pub async fn get_customer(&self, id: &String) -> Result<Option<SingleCustomerResponse>> {
    //     let query_result = sqlx::query_as!(
    //         CustomerModel,
    //         "SELECT * FROM customer WHERE customer_name=$1",
    //         id,
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    //     .map_err(SqlxError)?;

    //     let customer_response = SingleCustomerResponse {
    //         name: query_result.customer_name.unwrap_or("john".to_string()),
    //         surname: query_result.customer_surname.unwrap_or("doe".to_string()),
    //         status: "success".to_string(),
    //     };

    //     Ok(Some(customer_response))
    // }

    fn model_to_result(&self, customer: &CustomerModel) -> Result<CustomerResponse> {
        let customer_response = CustomerResponse {
            name: customer.customer_name.to_owned().unwrap(),
            surname: customer.customer_surname.to_owned().unwrap(),
        };

        Ok(customer_response)
    }

}
