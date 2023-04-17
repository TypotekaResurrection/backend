use async_graphql::{Context, Object, Result};
use entity::{article, async_graphql, category, category_article, comment, sea_orm, sea_orm::EntityTrait, user};
use entity::async_graphql::Error;
use entity::sea_orm::{EntityOrSelect, QuerySelect};
use entity::sea_orm::PaginatorTrait;
use serde::{Deserialize, Serialize};
use entity::sea_orm::prelude::DateTime;
use async_graphql::*;

use crate::db::Database;
use crate::graphql::query::comment::NormalComment;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(Default)]
pub struct ArticleQuery;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "Article", params()))]
pub struct NormalArticle {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub date: DateTime,
    pub preview: String,
    pub text: String,
    pub image_url: String,
    pub user_id: i32,
    pub categories: Vec<String>,
}

async fn transform_article(article: article::Model, db: &Database) -> Result<NormalArticle> {
    let user = user::Entity::find_by_id(article.user_id)
        .one(db.get_connection())
        .await
        .map_err(|e| e.to_string())?
        .unwrap();
    let category_articles = category_article::Entity::find_by_article_id(article.id)
        .all(db.get_connection())
        .await
        .map_err(|e| e.to_string())?;
    let mut categories = vec![];
    for category_article in category_articles {
        let category = category::Entity::find_by_id(category_article.category_id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?
            .unwrap();
        categories.push(category.name);
    }
    let article = NormalArticle {
        id: article.id,
        title: article.title,
        date: article.date,
        preview: article.preview,
        text: article.text,
        image_url: article.image_url,
        user_id: article.user_id,
        categories,
    };
    Ok(article)
}

#[Object]
impl ArticleQuery {
    async fn get_articles(&self, ctx: &Context<'_>) -> Result<Vec<NormalArticle>> {
        let db = ctx.data::<Database>().unwrap();

        let articles = article::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;
        let mut normal_articles = vec![];
        for articles in articles {
            let article = transform_article(articles, db).await?;
            normal_articles.push(article);
        }
        Ok(normal_articles)
    }

    async fn get_hot_articles(&self, ctx: &Context<'_>, limit: u32) -> Result<Vec<NormalArticle>> {
        let db = ctx.data::<Database>().unwrap();
        let mut article_comment_counts = Vec::new();
        let articles = article::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;
        for article in articles {
            let count = comment::Entity::find_by_article_id(article.id)
                .count(db.get_connection())
                .await?;
            article_comment_counts.push((article, count));
        }
        article_comment_counts.sort_by(|a, b| b.1.cmp(&a.1));
        article_comment_counts.truncate(limit as usize);
        let hot_articles = article_comment_counts
            .iter()
            .map(|(article, _)| article.clone())
            .collect::<Vec<article::Model>>();
        let mut normal_articles = vec![];
        for articles in hot_articles {
            let article = transform_article(articles, db).await?;
            normal_articles.push(article);
        }
        Ok(normal_articles)
    }

    async fn get_article_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<NormalArticle>> {
        let db = ctx.data::<Database>().unwrap();

        let article = article::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?.unwrap();
        let article = transform_article(article, db).await?;
        Ok(Some(article))
    }
    async fn get_articles_by_user(&self, ctx: &Context<'_>) -> Result<Option<NormalArticle>> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>();
        let res = validate_token(token.unwrap().token.as_str());
        if let Err(e) = res {
            return Err(Error::new(e.to_string()));
        }
        let article = article::Entity::find_by_user_id(dbg!(res?.id))
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?.unwrap();
        let normal_article = transform_article(article, db).await?;
        Ok(Some(normal_article))
    }

    async fn find_articles_by_title(&self, ctx: &Context<'_>, title: String) -> Result<Vec<article::Model>> {
        let db = ctx.data::<Database>().unwrap();

        let articles = article::Entity::find_by_title(title.as_str())
            .all(db.get_connection())
            .await?;
        Ok(articles)
    }
}
