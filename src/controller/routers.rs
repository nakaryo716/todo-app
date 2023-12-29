use axum::Router;

use crate::services::statics::static_file;

pub fn app() -> Router {
    Router::new()
    .fallback_service(static_file())
}