use async_graphql::{Context, Object, Result};
use dotenv::Error;
use entity::image;
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::sea_orm::{ActiveModelTrait, Set};

use crate::db::Database;
use crate::graphql::mutation::delete_result::DeleteResult;
use crate::utils::auth::Token;
use crate::utils::jwt::validate_token;

#[derive(InputObject)]
pub struct InputImage {
    pub url: String,
    pub alt: String,
}

#[derive(Default)]
pub struct ImageMutation;

async fn get_binary(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let buf = res.bytes().await?.to_vec();
    Ok(buf)
}

#[Object]
impl ImageMutation {
    pub async fn create_image(&self, ctx: &Context<'_>, input: InputImage) -> Result<image::Model> {
        let db = ctx.data::<Database>().unwrap();

        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        let claims = res.unwrap();

        let image = image::ActiveModel {
            binary: Set(get_binary(input.url.as_str()).await.unwrap()),
            alt: Set(input.alt),
            ..Default::default()
        };
        Ok(image.insert(db.get_connection()).await?)
    }

    pub async fn delete_image(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let token = ctx.data::<Token>()?;

        let res = validate_token(token.token.as_str());
        if let Err(error) = res {
            return Err(error.into());
        }
        let claims = res.unwrap();

        let res = image::Entity::delete_by_id(id).exec(db.get_connection()).await?;
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



