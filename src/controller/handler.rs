use std::sync::Arc;
use axum::{response::{IntoResponse, Html}, Json, extract::State};
use hyper::StatusCode;
use crate::model::{data_type::CreateTodo, repository::TodoRepositoryForMemory};

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Hello World!</h1>")
}

pub async fn create_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>, 
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}