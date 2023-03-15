use async_graphql::{Context, Object, Result, Error};
use entity::user;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, PaginatorTrait, Set, EntityTrait};

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;

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
    pub async fn create_user(&self, ctx: &Context<'_>, input: RegisterInput, ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();

        if input.password != input.password_confirmation {
            return Err(Error::new("Passwords do not match"));
        }

        let user = user::Entity::find_by_email(&input.email)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?;

        if let Some(_user) = user {
            return Err(Error::new(stringify!("User with {} email already exists", user.email)));
        };

        let is_staff = user::Entity::find().count(db.get_connection()).await? == 0;

        let user = user::ActiveModel {
            first_name: Set(input.first_name),
            last_name: Set(input.last_name),
            email: Set(input.email),
            password: Set(input.password),
            is_active: Set(true),
            is_staff: Set(is_staff),
            ..Default::default()
        };

        Ok(user.insert(db.get_connection()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

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