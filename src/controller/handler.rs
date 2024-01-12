use axum::response::{IntoResponse, Html};

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Hello World!</h1>")
}