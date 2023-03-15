use async_graphql::{Context, Object, Result};
use entity::{article, category_article, category};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};
use chrono::NaiveDateTime;

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateArticleInput {
    pub title: String,
    pub text: String,
    pub preview: String,
    pub category_ids: Vec<i32>,
    pub user_id: i32,
}


#[derive(Default)]
pub struct ArticleMutation;

#[Object]
impl ArticleMutation {
    pub async fn create_article(&self, ctx: &Context<'_>, input: CreateArticleInput) -> Result<article::Model> {
        let db = ctx.data::<Database>().unwrap();

        let article = article::ActiveModel {
            title: Set(input.title),
            text: Set(input.text),
            preview: Set(input.preview),
            date: Set(NaiveDateTime::default()),
            user_id: Set(input.user_id),
            ..Default::default()
        };
        let res = article.insert(db.get_connection()).await;
        if let Err(e) = &res {
            return Err(e.into());
        }
        let res = res.unwrap();
        for category_id in input.category_ids {
            let category_article = category_article::ActiveModel {
                article_id: Set(res.id),
                category_id: Set(category_id),
            };
            category_article.insert(db.get_connection()).await?;
        }
        println!("article");
        Ok(res)
    }

    pub async fn delete_article(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = article::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
