use axum::Router;

use crate::handlers::statics::static_files;

pub fn app() -> Router {
    Router::new()
    .fallback_service(static_files())
}
