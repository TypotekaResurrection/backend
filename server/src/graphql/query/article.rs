use async_graphql::{Context, Object, Result};
use entity::{article, async_graphql, sea_orm::EntityTrait, user};
use entity::async_graphql::Error;

use crate::db::Database;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(Default)]
pub struct ArticleQuery;

#[Object]
impl ArticleQuery {
    async fn get_articles(&self, ctx: &Context<'_>) -> Result<Vec<article::Model>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(article::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_article_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<article::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(article::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
    async fn get_articles_by_user(&self, ctx: &Context<'_>) -> Result<Option<article::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>();
        let res = validate_token(token.unwrap().token.as_str());
        if let Err(e) = res {
            return Err(Error::new(e.to_string()));
        }
        Ok(article::Entity::find_by_user_id(dbg!(res?.id))
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
