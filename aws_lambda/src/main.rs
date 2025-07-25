use std::env;

use aws_lambda::state::AppContext;
use axum::{Router, routing};
use lambda_http::{Error, run, tracing};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in env");
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    let ctx = AppContext { conn };
    let router = Router::new()
        .route(
            "/api/customers",
            routing::post(aws_lambda::handlers::create_customer),
        )
        .with_state(ctx);

    run(router).await
}
