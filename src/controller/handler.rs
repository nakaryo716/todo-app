use crate::{
    error::RepositoryError,
    model::{
        data_type::{CreateTodo, UpdateTodo, ValidatedJson},
        repository::TodoRepositoryForMemory,
    },
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
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .map_err(|err| {
            match err {
                RepositoryError::NotFound => StatusCode::NOT_FOUND,
                RepositoryError::Unexpected => StatusCode::SERVICE_UNAVAILABLE,
                RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn find_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.map_err(|err| match err {
        RepositoryError::NotFound => StatusCode::NOT_FOUND,
        RepositoryError::Unexpected => StatusCode::SERVICE_UNAVAILABLE,
        RepositoryError::DatabaseError(_text) => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .all()
        .await
        .map_err(|err| {
            match err {
                RepositoryError::NotFound => StatusCode::NOT_FOUND,
                RepositoryError::Unexpected => StatusCode::SERVICE_UNAVAILABLE,
                RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .await
        .map_err(|err| {
            match err {
                RepositoryError::NotFound => StatusCode::NOT_FOUND,
                RepositoryError::Unexpected => StatusCode::SERVICE_UNAVAILABLE,
                RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepositoryForMemory>(
    State(repository): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    repository
        .delete(id)
        .await
        .map_err(|err| {
            match err {
                RepositoryError::NotFound => StatusCode::NOT_FOUND,
                RepositoryError::Unexpected => StatusCode::SERVICE_UNAVAILABLE,
                RepositoryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
