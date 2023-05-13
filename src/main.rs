mod db_connection;
mod error;
mod handler;
mod model;
mod router;
use router::product::route;
use sqlx::{Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum-sqlx-postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let pool = db_connection::connect().await;
    let app = route(Arc::new(AppState { db: pool.clone() })).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::debug!("->> LISTENING ON {}", addr);
}

pub struct AppState {
    db: Pool<Postgres>,
}
