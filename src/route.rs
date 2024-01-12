use axum::{Router, routing::get};

use crate::controller::handler::hello;

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello))
}