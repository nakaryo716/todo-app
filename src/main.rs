use router::app;
use tracing::info;

mod db;
mod handlers;
mod middleware;
mod web;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = app();

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on {:?}", listner);

    info!("server was started");
    axum::serve(listner, app).await.unwrap()

   
}
