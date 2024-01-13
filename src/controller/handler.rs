use std::sync::Arc;
use axum::{response::{IntoResponse, Html}, Json, extract::{State, Path}};
use hyper::StatusCode;
use crate::model::{data_type::{CreateTodo, UpdateTodo}, repository::TodoRepositoryForMemory};

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

pub async fn find_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32> 
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .find(id)
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>
) -> impl IntoResponse {
    let todo = repository.all();

    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, StatusCode> {
    let _todo = repository
        .delete(id)
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok(StatusCode::NO_CONTENT)
}
