use async_graphql::{Context, Object, Result};
use entity::{article, category_article, category, comment, sea_orm, user};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use chrono::{Local, NaiveDateTime};
use chrono::format::Item::Error;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::ColumnTrait;

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateArticleInput {
    pub title: String,
    pub text: String,
    pub preview: String,
    pub category_ids: Vec<i32>,
    pub image_url: String,
}

#[derive(InputObject)]
pub struct UpdateArticleInput {
    pub title: Option<String>,
    pub text: Option<String>,
    pub preview: Option<String>,
    pub category_ids: Option<Vec<i32>>,
    pub image_url: Option<String>,
}


#[derive(Default)]
pub struct ArticleMutation;

#[Object]
impl ArticleMutation {
    pub async fn create_article(&self, ctx: &Context<'_>, input: CreateArticleInput) -> Result<article::Model> {
        let db = ctx.data::<Database>().unwrap();

        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        let claims = res.unwrap();

        let user = user::Entity::find_by_id(claims.id).one(db.get_connection()).await?;

        if user.is_none() {
            return Err(async_graphql::Error::new("User has been deleted"));
        }
        if !user.unwrap().is_staff {
            return Err(async_graphql::Error::new("Permission denied"));
        }

        let article = article::ActiveModel {
            title: Set(input.title),
            text: Set(input.text),
            preview: Set(input.preview),
            date: Set(Local::now().naive_local()),
            user_id: Set(claims.id),
            image_url: Set(input.image_url),
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
        println!("article created");
        Ok(res)
    }

    pub async fn update_article(&self, ctx: &Context<'_>, id: i32, input: UpdateArticleInput) -> Result<article::Model> {
        let db = ctx.data::<Database>().unwrap();
        let article = article::Entity::find_by_id(id)
            .one(db.get_connection())
            .await?
            .ok_or("Record not found").unwrap();
        let mut article: article::ActiveModel = article.into();

        if let Some(title) = input.title {
            article.title = Set(title);
        }
        if let Some(preview) = input.preview {
            article.preview = Set(preview);
        }
        if let Some(text) = input.text {
            article.text = Set(text);
        }
        if let Some(image_url) = input.image_url {
            article.image_url = Set(image_url);
        }

        if let Some(category_ids) = input.category_ids {
            for category_article in category_article::Entity::find_by_article_id(id).all(db.get_connection()).await? {
                category_article.delete(db.get_connection()).await?;
            }
            for category_id in category_ids {
                let category_article = category_article::ActiveModel {
                    article_id: Set(id),
                    category_id: Set(category_id),
                };
                category_article.insert(db.get_connection()).await?;
            }
        }

        let updated_article = article.update(db.get_connection()).await?;
        Ok(updated_article)
    }

    pub async fn delete_article(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>()?;
        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        //deleting related entities
        comment::Entity::delete_many()
            .filter(comment::Column::ArticleId.eq(id))
            .exec(db.get_connection())
            .await?;
        category_article::Entity::delete_many()
            .filter(category_article::Column::ArticleId.eq(id))
            .exec(db.get_connection())
            .await?;
        //deleting article
        let article = article::Entity::find_by_id(id).one(db.get_connection()).await?.unwrap();
        let res = article.delete(db.get_connection()).await?;
        println!("article deleted");
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
