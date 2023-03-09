use async_graphql::{Context, Object, Result};
use entity::category_article;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;

#[derive(InputObject)]
pub struct AddCategoryArticleInput {
    pub category_id: i32,
    pub article_id: i32,
}

#[derive(Default)]
pub struct CategoryArticleMutation;

#[Object]
impl CategoryArticleMutation {
    pub async fn add_category_article(&self, ctx: &Context<'_>, input: AddCategoryArticleInput)
        -> Result<category_article::Model> {
        let db = ctx.data::<Database>().unwrap();

        let category = category_article::ActiveModel {
            category_id: Set(input.category_id),
            article_id: Set(input.article_id),
        };
        Ok(category.insert(db.get_connection()).await?)
    }
}
