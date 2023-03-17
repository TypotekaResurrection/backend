use axum::{
    async_trait,
    extract::{Extension, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use entity::sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
struct AppState {
    token: Option<String>,
}

#[derive(Default, Debug)]
pub struct AuthenticatedUser {
    pub id: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map(|token| token.token().to_string())
                .unwrap_or_default();

        println!("token: {}", token);

        Ok(AuthenticatedUser {
            id: 1
        })
    }
}