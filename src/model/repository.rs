use crate::error::RepositoryError;
use async_trait::async_trait;
use sqlx::PgPool;

use super::data_type::{CreateTodo, Todo, UpdateTodo};

#[async_trait]
pub trait TodoRepositoryForMemory: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError>;
    async fn find(&self, id: i32) -> Result<Todo, RepositoryError>;
    async fn all(&self) -> Result<Vec<Todo>, RepositoryError>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        TodoRepository { pool }
    }
}

#[async_trait]
impl TodoRepositoryForMemory for TodoRepository {
    async fn create(&self, payload: CreateTodo) -> Result<Todo, RepositoryError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
INSERT INTO todos (text, completed)
VALUES ($1, false)
RETURNING *
            "#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::Protocol(text) => RepositoryError::DatabaseError(text),
                _ => RepositoryError::Unexpected,
            }
        })?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> Result<Todo, RepositoryError> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
SELECT * FROM todos WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::Unexpected,
        })?;

        Ok(todo)
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>, RepositoryError> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
SELECT * FROM todos
ORDER BY id DESC;      
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                sqlx::Error::Protocol(text) => RepositoryError::DatabaseError(text),
                _ => RepositoryError::Unexpected,
            }
        })?;

        Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, RepositoryError> {
        let old_todo = self.find(id).await?;

        let text = if payload.text.is_none() {
            old_todo.text
        } else {
            payload.text.unwrap()
        };

        let completed = if payload.completed.is_none() {
            old_todo.completed
        } else {
            payload.completed.unwrap()
        };

        let todo = sqlx::query_as::<_, Todo>(
            r#"
UPDATE todos SET text = $1, completed = $2
WHERE id = $3
RETURNING *       
        "#,
        )
        .bind(text)
        .bind(completed)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::Protocol(text) => RepositoryError::DatabaseError(text),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(todo)
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let _todo = sqlx::query(
            r#"
DELETE FROM todos WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::Protocol(text) => RepositoryError::DatabaseError(text),
            _ => RepositoryError::Unexpected,
        })?;

        Ok(())
    }
}
