use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn connect() -> Pool<Postgres> {
    let database_uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_uri)
        .await
        .unwrap();
    pool
}
