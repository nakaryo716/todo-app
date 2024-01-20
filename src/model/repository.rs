use crate::error::RepositoryError;
use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;

use super::data_type::{CreateTodo, Todo, UpdateTodo};

#[async_trait]
pub trait TodoRepositoryForMemory: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo, RepositoryError>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
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
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
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
        .context(RepositoryError::Unexpected)?;

        anyhow::Ok(todo)
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

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
SELECT * FROM todos
ORDER BY id DESC;      
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context(RepositoryError::Unexpected)?;

        anyhow::Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
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
            _ => RepositoryError::Unexpected,
        })?;

        anyhow::Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
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
            _ => RepositoryError::Unexpected,
        })?;

        anyhow::Ok(())
    }
}
