use route::app;
use std::env;

use crate::model::repository::TodoRepository;

mod controller;
mod error;
mod model;
mod route;
mod services;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepository::new();

    let app = app(repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", &listener);

    tracing::debug!("server start");
    axum::serve(listener, app).await.unwrap();
}
