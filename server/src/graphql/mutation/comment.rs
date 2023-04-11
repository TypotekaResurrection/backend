use async_graphql::{Context, Object, Result};
use chrono::{Local, NaiveDateTime};
use entity::comment;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(InputObject)]
pub struct CreateCommentInput {
    pub article_id: i32,
    pub content: String,
}


#[derive(Default)]
pub struct CommentMutation;

#[Object]
impl CommentMutation {
    pub async fn create_comment(&self, ctx: &Context<'_>, input: CreateCommentInput, ) -> Result<comment::Model> {
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        let claims = res.unwrap();

        let comment = comment::ActiveModel {
            article_id: Set(input.article_id),
            user_id: Set(claims.id),
            content: Set(input.content),
            date: Set(Local::now().naive_local()),
            ..Default::default()
        };

        Ok(comment.insert(db.get_connection()).await?)
    }

    pub async fn delete_comment(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }

        let res = comment::Entity::delete_by_id(id)
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


