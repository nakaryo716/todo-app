use std::sync::Arc;

use crate::{
    controller::handler::{all_todo, create_todo, delete_todo, find_todo, hello, update_todo},
    model::repository::TodoRepositoryForMemory,
};
use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

pub fn app<T: TodoRepositoryForMemory>(repository: T) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/todo", post(create_todo).get(all_todo))
        .route(
            "/todo/:id",
            get(find_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}
