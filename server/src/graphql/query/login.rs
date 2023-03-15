use async_graphql::{Context, Object, Result, Error};
use axum::Json;
use entity::user::Entity as User;
use entity::user;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};

use crate::db::Database;
use crate::{KEYS};
use utils::jwt::get_timestamp_8_hours_from_now;
use utils::jwt::Claims;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Default)]
pub struct AuthQuery;



impl AuthQuery {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<LoginResponse> {
        let db = ctx.data::<Database>().unwrap();

        let user = User::find_by_email(&input.email)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(_) = user {
            let claims = Claims {
                email: input.email.to_owned(),
                exp: get_timestamp_8_hours_from_now(),
            };
            let token = encode(&Header::default(), &claims, &KEYS.encoding)?;
            // return bearer token
            Ok(LoginResponse {
                access_token: token,
                token_type: "Bearer".to_string(),
            })
            } else {
                Err(Error::new("User does not exist"))
            }
        }
    }
