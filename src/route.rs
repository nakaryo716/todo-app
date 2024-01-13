use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use crate::{controller::handler::{hello, create_todo, all_todo, find_todo, delete_todo, update_todo}, model::repository::TodoRepositoryForMemory};

pub fn app<T: TodoRepositoryForMemory>(repository: T) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/todo", post(create_todo).get(all_todo))
        .route("/todo/:id", get(find_todo).put(update_todo).delete(delete_todo))
        .with_state(Arc::new(repository))
}