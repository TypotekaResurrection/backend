use crate::db::Database;
use entity::{user, async_graphql, sea_orm::EntityTrait};
use async_graphql::{Context, Object, Result, Error};
use entity::async_graphql::{InputObject, SimpleObject};

#[derive(Default)]
pub struct LoginQuery;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[Object]
impl LoginQuery {
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<String> {
        let db = ctx.data::<Database>().unwrap();

        let user = user::Entity::find_by_email(email.as_str())
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(user) = user {
            if user.password == password {
                let token = jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &user,
                    &jsonwebtoken::EncodingKey::from_secret("1234".as_ref()),
                )
                .unwrap();

                Ok(token)
            } else {
                Err(Error::new("Invalid password"))
            }
        } else {
            Err(Error::new("User not found"))
        }
    }
}