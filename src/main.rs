use dotenv::dotenv;
use route::app;
use sqlx::postgres::PgPoolOptions;
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

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("notfound database url");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("cannot connect database");

    let repository = TodoRepository::new(pool.clone());

    let app = app(repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", &listener);

    tracing::debug!("server start");
    axum::serve(listener, app).await.unwrap();
}
