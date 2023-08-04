use crate::response::{CustomerData, CustomerResponse, SingleCustomerResponse,};
use crate::{
    model::CustomerModel, Result, schema::CreateCustomerSchema
};
use sqlx::postgres::PgPool;
use sqlx::{self, Postgres};



#[derive(Clone, Debug)]
pub struct PgDB {
    pub connection: PgPool,
}

impl PgDB {
    pub async fn init() -> Self { 
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

        let pool = match PgPool::connect(&pg_uri).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection:[]", e)
        };
        println!("✅ Database connected successfully");

         PgDB{connection: pool,}
    } 

    pub async fn create_customer(&mut self, body: &CreateCustomerSchema) -> Result<Option<SingleCustomerResponse>> {
        let customer_name = body.customer_name.to_owned();
        let customer_surname = body.customer_surname.to_owned();

        let response = sqlx::query_as!(CustomerModel,
                "INSERT INTO customers (customer_name, customer_surname) VALUES ($1, $2)", 
                customer_name, 
                customer_surname).fetch_one(&self.pool).await?;

        let customer = CustomerResponse{
            customer_name:customer_name,
            customer_surname:customer_surname
        };

        let customer_response = SingleCustomerResponse {
            status: "success".to_string(),
            data: CustomerData {
                customer: customer,
            },
        };

        Ok(Some(customer_response))
    }


    fn doc_to_note(&self, note: &CustomerModel) -> Result<CustomerResponse> {
        let customer_response = CustomerResponse {
            customer_name: note.customer_name.to_owned(),
            customer_surname: note.customer_surname.to_owned(),
        };

        Ok(customer_response)
    }

}
