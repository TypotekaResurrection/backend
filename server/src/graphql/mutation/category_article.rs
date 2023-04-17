// use async_graphql::{Context, Object, Result};
// use entity::{category_article, user};
// use entity::async_graphql::{self, InputObject, SimpleObject};
// use entity::sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait, ModelTrait};
//
// use crate::db::Database;
// use crate::graphql::mutation::delete_result::DeleteResult;
// use crate::utils::auth::Token;
// use crate::utils::jwt::validate_token;
//
// #[derive(InputObject)]
// pub struct CategoryArticleInput {
//     pub category_id: i32,
//     pub article_id: i32,
// }
//
// #[derive(Default)]
// pub struct CategoryArticleMutation;
//
// #[Object]
// impl CategoryArticleMutation {
//     pub async fn add_category_article(&self, ctx: &Context<'_>, input: CategoryArticleInput) -> Result<category_article::Model> {
//         let db = ctx.data::<Database>().unwrap();
//
//         let token = ctx.data::<Token>()?;
//
//         let res = validate_token(token.token.as_str());
//         if let Err(error) = res {
//             return Err(error.into());
//         }
//         let claims = res.unwrap();
//         let user = user::Entity::find_by_id(claims.id).one(db.get_connection()).await?;
//         if user.is_none() {
//             return Err(async_graphql::Error::new("User has been deleted"));
//         }
//         if let Some(user) = user {
//             if !user.is_staff {
//                 return Err(async_graphql::Error::new("Permission denied"));
//             }
//         }
//         let category = category_article::ActiveModel {
//             category_id: Set(input.category_id),
//             article_id: Set(input.article_id),
//         };
//         Ok(category.insert(db.get_connection()).await?)
//     }
//     pub async fn delete_category_article_by_article_id(&self, ctx: &Context<'_>, input: CategoryArticleInput) -> Result<DeleteResult> {
//         let db = ctx.data::<Database>().unwrap();
//
//         let token = ctx.data::<Token>()?;
//
//         let res = validate_token(token.token.as_str());
//         if let Err(error) = res {
//             return Err(error.into());
//         }
//
//         let category_article = category_article::Entity::find()
//             .filter(category_article::Column::ArticleId.eq(input.article_id))
//             .filter(category_article::Column::CategoryId.eq(input.category_id))
//             .one(db.get_connection())
//             .await?.unwrap();
//         let res = category_article.delete(db.get_connection()).await?;
//
//         if res.rows_affected <= 1 {
//             Ok(DeleteResult {
//                 success: true,
//                 rows_affected: res.rows_affected,
//             })
//         } else {
//             unimplemented!()
//         }
//     }
// }
