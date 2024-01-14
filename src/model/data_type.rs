use async_trait::async_trait;
use axum::{extract::{FromRequest, rejection::JsonRejection, Request}, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100,  message = "Over length"))]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);


#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|err|{
            let message = format!("Json parse error: {}", err);

            (StatusCode::BAD_REQUEST, message)
        })?;

        value.validate().map_err(|err| {
            let message = format!("Json parse error: {}", err);

            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

