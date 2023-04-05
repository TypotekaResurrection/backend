use async_graphql::{Context, Object, Result};
use entity::{user, async_graphql, sea_orm::EntityTrait};
use async_graphql::InputObject;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use entity::async_graphql::Error;
use crate::utils::jwt::validate_token;

use crate::db::Database;
use crate::KEYS;
use crate::utils::auth::Token;
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

    async fn get_user_info(&self, ctx: &Context<'_>, ) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>();
        let res = validate_token(token.unwrap().token.as_str());
        println!("{:?}", res);
        if let Err(e) = res {
            return Err(Error::new(e.to_string()));
        }

        let claims = res.unwrap();

        Ok(user::Entity::find_by_id(claims.id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data::<Database>().unwrap();

        let user = user::Entity::find_by_email(input.email.as_str())
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(user) = user {
            if user.password == input.password {
                let claims = Claims {
                    id: user.id,
                    exp: get_timestamp_8_hours_from_now(),
                };
                let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();
                Ok(token)
            } else {
                Err(Error::new("Invalid credentials"))
            }
        } else {
            Err(Error::new("User not found"))
        }
    }
}