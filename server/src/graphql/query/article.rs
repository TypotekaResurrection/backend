use async_graphql::{Context, Object, Result};
use entity::{article, async_graphql, sea_orm::EntityTrait};

use crate::db::Database;

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

    async fn get_article_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<article::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(article::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
