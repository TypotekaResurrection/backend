use async_graphql::{Context, Object, Result};
use entity::{article, async_graphql, comment, sea_orm, sea_orm::EntityTrait, user};
use entity::async_graphql::Error;
use entity::sea_orm::{EntityOrSelect, QuerySelect};
use entity::sea_orm::PaginatorTrait;

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

    async fn get_hot_articles(&self, ctx: &Context<'_>, limit: u32) -> Result<Vec<article::Model>> {
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
        Ok(hot_articles)
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

    async fn find_articles_by_title(&self, ctx: &Context<'_>, title: String) -> Result<Vec<article::Model>> {
        let db = ctx.data::<Database>().unwrap();

        let articles = article::Entity::find_by_title(title.as_str())
            .all(db.get_connection())
            .await?;
        Ok(articles)
    }
}
