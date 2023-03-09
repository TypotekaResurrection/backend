use async_graphql::{Context, Object, Result};
use entity::{comment, async_graphql, sea_orm::EntityTrait};

use crate::db::Database;

#[derive(Default)]
pub struct CommentQuery;

#[Object]
impl CommentQuery {
    async fn get_comments(&self, ctx: &Context<'_>) -> Result<Vec<comment::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(comment::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_comment_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<comment::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(comment::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
