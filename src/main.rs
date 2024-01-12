use tracing::info;
use route::app;

mod model;
mod controller;
mod route;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = app();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    info!("listening on {:?}", &listener);

    info!("server start");
    
    axum::serve(listener, app).await.unwrap();
}
