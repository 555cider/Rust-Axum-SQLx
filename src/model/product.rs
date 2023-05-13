use crate::{error::CustomError, AppState};
use axum::{extract::State, Json};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>,
    pub created_at: Option<DateTime<Local>>,
}

#[derive(Deserialize)]
pub struct NewProduct {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>,
}

impl NewProduct {
    pub async fn create(
        State(data): State<Arc<AppState>>,
        Json(new_product): Json<NewProduct>,
    ) -> Result<(), CustomError> {
        let query = sqlx::query(
            "INSERT INTO axum.product (name, stock, price) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(new_product.name)
        .bind(new_product.stock)
        .bind(new_product.price);
        let _product = query.fetch_one(&data.db).await.unwrap();
        Ok(())
    }

    pub async fn update(
        State(data): State<Arc<AppState>>,
        Json(new_product): Json<NewProduct>,
    ) -> Result<(), CustomError> {
        let query = sqlx::query(
            "UPDATE axum.product SET name = $1, stock = $2, price = $3 WHERE id = $4 RETURNING *",
        )
        .bind(new_product.name)
        .bind(new_product.stock)
        .bind(new_product.price)
        .bind(new_product.id);
        let _product = query.fetch_one(&data.db).await.unwrap();
        Ok(())
    }
}

impl Product {
    pub async fn list(State(data): State<Arc<AppState>>) -> Result<Vec<Product>, CustomError> {
        let query = sqlx::query_as("SELECT * FROM axum.product");
        let product_list = query.fetch_all(&data.db).await.unwrap();
        Ok(product_list)
    }

    pub async fn get(State(data): State<Arc<AppState>>, id: i32) -> Result<Product, CustomError> {
        let query =
            sqlx::query_as::<_, Product>("SELECT * FROM axum.product WHERE id = $1").bind(id);
        let product = query.fetch_one(&data.db).await.unwrap();
        Ok(product)
    }

    pub async fn delete(State(data): State<Arc<AppState>>, id: i32) -> Result<(), CustomError> {
        let query = sqlx::query("DELETE FROM axum.product WHERE id = $1 RETURNING *").bind(id);
        query.execute(&data.db).await.unwrap();
        Ok(())
    }
}
