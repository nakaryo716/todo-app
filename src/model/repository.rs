use anyhow::Context;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::error::RepositoryError;

use super::data_type::{CreateTodo, Todo, UpdateTodo};

pub trait TodoRepositoryForMemory: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> anyhow::Result<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

type TodoData = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepository {
    pool: Arc<RwLock<TodoData>>,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            pool: Arc::default(),
        }
    }
}

impl TodoRepositoryForMemory for TodoRepository {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.pool.write().unwrap();

        let id = (store.len() as i32) + 1;
        let todo = Todo::new(id, payload.text);
        store.insert(id, todo.clone());

        todo
    }

    fn find(&self, id: i32) -> anyhow::Result<Todo> {
        let store = self.pool.read().unwrap();

        let todo = store
            .get(&id)
            .map(|m| m.clone())
            .context(RepositoryError::NotFound(id))?;

        anyhow::Ok(todo)
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.pool.read().unwrap();
        let mut todos = Vec::new();

        for (_id, value) in store.clone().into_iter() {
            todos.push(value);
        }

        todos
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.pool.write().unwrap();
        let todo = store
            .get(&id)
            .map(|todo| todo.clone())
            .context(RepositoryError::NotFound(id))?;

        let text = if payload.text.is_none() {
            todo.text
        } else {
            payload.text.unwrap()
        };

        let completed = if payload.completed.is_none() {
            todo.completed
        } else {
            payload.completed.unwrap()
        };

        let todo = Todo {
            id,
            text,
            completed,
        };

        store.insert(id, todo.clone());

        anyhow::Ok(todo)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.pool.write().unwrap();
        store.remove(&id).context(RepositoryError::NotFound(id))?;

        anyhow::Ok(())
    }
}
