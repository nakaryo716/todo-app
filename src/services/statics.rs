use axum::{Router, routing::get_service};

use tower_http::services::ServeDir;

pub fn static_file() -> Router {
    Router::new()
    .nest_service("/static", get_service(ServeDir::new("./static")))
}

