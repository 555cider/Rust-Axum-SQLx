use crate::model::product::{NewProduct, Product};
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub async fn list_product(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Json<Vec<Product>>> {
    let product_list = Product::list(State(data)).await.unwrap();
    Ok(Json(product_list))
}

pub async fn get_product(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, Json<Product>> {
    let product = Product::get(State(data), id).await;
    Ok(Json(product))
}

pub async fn create_product(
    State(data): State<Arc<AppState>>,
    Json(new_product): Json<NewProduct>,
) -> Result<impl IntoResponse, Json<Product>> {
    let product = NewProduct::create(State(data), Json(new_product)).await;
    Ok(Json(product))
}

pub async fn update_product(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut new_product): Json<NewProduct>,
) -> Result<impl IntoResponse, Json<Product>> {
    new_product.id = Some(id);
    let product = NewProduct::update(State(data), Json(new_product)).await;
    Ok(Json(product))
}

pub async fn delete_product(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ()> {
    let result = Product::delete(State(data), id);
    Ok(result.await)
}
