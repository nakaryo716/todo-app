use axum::Router;
use tower_http::services::ServeDir;

pub fn static_files() -> Router {
    Router::new()
    .nest_service("/static", ServeDir::new("static"))
}