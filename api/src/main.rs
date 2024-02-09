use app::model::repository::TodoRepository;
use app::routes::route;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("notfound database url");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("cannot connect database");

    let repository = TodoRepository::new(pool.clone());

    let app = route::app(repository);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {:?}", &listener);

    tracing::info!("server start");
    axum::serve(listener, app).await.unwrap();
}
