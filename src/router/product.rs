use crate::{
    handler::product::{self, create_product},
    AppState,
};
use axum::{
    routing::{get, get_service},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

pub async fn route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/product", get(product::list_product).post(create_product))
        .route(
            "/product/:id",
            get(product::get_product)
                .put(product::update_product)
                .delete(product::delete_product),
        )
        .with_state(app_state)
        .fallback_service(get_service(ServeDir::new("./")))
}
