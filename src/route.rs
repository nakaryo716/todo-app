use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use crate::{controller::handler::{hello, create_todo}, model::repository::TodoRepositoryForMemory};

pub fn app<T: TodoRepositoryForMemory>(repository: T) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/todo", post(create_todo))
        .with_state(Arc::new(repository))
}