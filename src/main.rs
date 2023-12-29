use controller::routers::app;
use tracing::info;

mod model;
mod controller;
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
