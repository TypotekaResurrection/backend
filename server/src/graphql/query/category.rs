use async_graphql::{Context, Object, Result};
use entity::{category, async_graphql, sea_orm::EntityTrait};

use crate::db::Database;


#[derive(Default)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    async fn get_categories(&self, ctx: &Context<'_>) -> Result<Vec<category::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(category::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<category::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(category::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}