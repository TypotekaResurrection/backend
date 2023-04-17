use async_graphql::{Context, Object, Result};
use entity::{comment, async_graphql, sea_orm::EntityTrait, article, user};

use crate::db::Database;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;


use async_graphql::*;
use serde::{Deserialize, Serialize};
use entity::sea_orm::prelude::DateTime;

#[derive(Default)]
pub struct CommentQuery;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "Comment", params()))]
pub struct NormalComment {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub date: DateTime,
    pub content: String,
    pub article_name: String,
    pub user_name: String,
}

pub async fn transform_comment(comment: comment::Model, db: &Database) -> Result<NormalComment> {
    let article_name = article::Entity::find_by_id(comment.article_id)
        .one(db.get_connection())
        .await
        .map_err(|e| e.to_string())?
        .unwrap()
        .title;
    let user = user::Entity::find_by_id(comment.user_id)
        .one(db.get_connection())
        .await
        .map_err(|e| e.to_string())?
        .unwrap();
    let user_name = user.last_name + " " +  user.first_name.as_str();
    let normal_comment = NormalComment {
        id: comment.id,
        date: comment.date,
        content: comment.content,
        article_name,
        user_name,
    };
    Ok(normal_comment)
}

#[Object]
impl CommentQuery {
    async fn get_comments(&self, ctx: &Context<'_>, limit: Option<usize>) -> Result<Vec<NormalComment>> {
        let db = ctx.data::<Database>().unwrap();

        let mut comment = comment::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        comment.sort_by(|a, b| b.date.cmp(&a.date));
        if limit.is_some() {
            let limit = limit.unwrap();
            comment.truncate(limit);
        }
        let mut normal_comments = vec![];
        for comment in comment {
            let normal_comment = transform_comment(comment, db).await?;
            normal_comments.push(normal_comment);
        }
        Ok(normal_comments)
    }

    async fn get_comment_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<comment::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(comment::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_comments_by_user_id(&self, ctx: &Context<'_>, limit: Option<usize>) -> Result<Vec<NormalComment>> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        let claims = res.unwrap();
        let mut comments = comment::Entity::find_by_user_id(claims.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;
        if comments.is_empty(){
            return Err("No comments".into());
        }
        comments.sort_by(|a, b| b.date.cmp(&a.date));
        if limit.is_some() {
            let limit = limit.unwrap();
            comments.truncate(limit);
        }
        let mut normal_comments = vec![];
        for comment in comments {
            let normal_comment = transform_comment(comment, db).await?;
            normal_comments.push(normal_comment);
        }
        Ok(normal_comments)
    }

    async fn get_comments_by_article_id(&self, ctx: &Context<'_>, article_id: i32, limit: Option<usize>) -> Result<Vec<NormalComment>> {
        let db = ctx.data::<Database>().unwrap();

        let mut comments = comment::Entity::find_by_article_id(article_id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        comments.sort_by(|a, b| a.date.cmp(&b.date));
        if limit.is_some() {
            let limit = limit.unwrap();
            comments.truncate(limit);
        }
        let mut normal_comments = vec![];
        for comment in comments {
            let normal_comment = transform_comment(comment, db).await?;
            normal_comments.push(normal_comment);
        }
        Ok(normal_comments)
    }
}


