use crate::model::{
    data_type::{CreateTodo, UpdateTodo, ValidatedJson},
    repository::TodoRepositoryForMemory,
};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Json,
};
use hyper::StatusCode;
use std::sync::Arc;

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Hello World!</h1>")
}

pub async fn create_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.all();

    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository.delete(id).or(Err(StatusCode::NOT_FOUND))?;

    Ok(StatusCode::NO_CONTENT)
}
