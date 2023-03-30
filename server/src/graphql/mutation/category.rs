use async_graphql::{Context, Object, Result};
use entity::{category, user};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(InputObject)]
pub struct CreateCategoryInput {
    pub name: String,
}

#[derive(Default)]
pub struct CategoryMutation;

#[Object]
impl CategoryMutation {
    pub async fn create_category(&self, ctx: &Context<'_>, input: CreateCategoryInput) -> Result<category::Model> {
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
        if let Some(user) = user {
            if !user.is_staff{
                return Err(async_graphql::Error::new("Permission denied"));
            }
        }

        let category = category::ActiveModel {
            name: Set(input.name),
            ..Default::default()
        };
        Ok(category.insert(db.get_connection()).await?)
    }

    pub async fn delete_category(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();


        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }

        let res = category::Entity::delete_by_id(id)
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