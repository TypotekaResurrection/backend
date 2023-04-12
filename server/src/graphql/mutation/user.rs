use async_graphql::{Context, Object, Result, Error};
use entity::{article, comment, user};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, EntityTrait, Set};
use entity::sea_orm::QueryFilter;
use entity::sea_orm::ColumnTrait;

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(InputObject)]
pub struct RegisterInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<user::Model> {
        println!("create_user: ");
        let db = ctx.data::<Database>().unwrap();
        let user = user::Entity::find_by_email(&input.email)
            .one(db.get_connection())
            .await?;

        if user.is_some() {
            return Err(Error::new("User with this email already exists"));
        }

        if input.password != input.password_confirmation {
            return Err(Error::new("Passwords do not match"));
        }


        let is_admin = user::Entity::find()
            .all(db.get_connection())
            .await?
            .is_empty();

        let user = user::ActiveModel {
            first_name: Set(input.first_name),
            last_name: Set(input.last_name),
            email: Set(input.email),
            password: Set(input.password),
            is_active: Set(true),
            is_staff: Set(is_admin),
            ..Default::default()
        };

        Ok(user.insert(db.get_connection()).await?)
    }

    pub async fn update_user(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<user::Model> {
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
                return Err(Error::new("Permission denied"));
            }
        }

        let user = user::ActiveModel {
            first_name: Set(input.first_name),
            last_name: Set(input.last_name),
            email: Set(input.email),
            password: Set(input.password),
            ..Default::default()
        };

        Ok(user.update(db.get_connection()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        //auth
        let db = ctx.data::<Database>().unwrap();
        let token = ctx.data::<Token>()?;
        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }

        // deleting related entities
        comment::Entity::delete_many()
            .filter(comment::Column::UserId.eq(id))
            .exec(db.get_connection())
            .await?;
        article::Entity::delete_many()
            .filter(article::Column::UserId.eq(id))
            .exec(db.get_connection())
            .await?;

        //deleting user
        let res = user::Entity::delete_by_id(id)
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