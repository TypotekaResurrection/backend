use async_graphql::{Context, Object, Result};
use entity::{user, async_graphql, sea_orm::EntityTrait};
use async_graphql::InputObject;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use crate::utils::jwt::validate_token;

use crate::db::Database;
use crate::KEYS;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::jwt::*;


#[derive(Default)]
pub struct UserQuery;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(user::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<Option<String>> {
        let db = ctx.data::<Database>().unwrap();
        println!("input: {:?}", input.email);
        println!("input: {:?}", input.password);

        let user = user::Entity::find_by_email(&input.email)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(user) = user {
            if user.password == input.password {
                let claims = Claims {
                    id: user.id,
                    exp: get_timestamp_8_hours_from_now(),
                };
                return Ok(Some(encode(&Header::default(), &claims, &KEYS.encoding).unwrap()));
            }
        }

        Ok(None)
    }
}