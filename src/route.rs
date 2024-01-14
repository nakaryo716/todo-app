use std::sync::Arc;

use crate::{
    controller::handler::{all_todo, create_todo, delete_todo, find_todo, hello, update_todo},
    model::repository::TodoRepositoryForMemory,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn app<T: TodoRepositoryForMemory>(repository: T) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/todo", post(create_todo).get(all_todo))
        .route(
            "/todo/:id",
            get(find_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(Arc::new(repository))
}
