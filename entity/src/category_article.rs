use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "category_articles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub category_id: i32,
    #[sea_orm(primary_key)]
    pub article_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_delete = "Cascade"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::article::Entity",
        from = "Column::ArticleId",
        to = "super::article::Column::Id",
        on_delete = "Cascade"
    )]
    Article,
}

impl ActiveModelBehavior for ActiveModel {}


impl Entity {
    pub fn find_by_category_id(category_id: i32) -> Select<Entity> {
        Self::find().filter(Column::CategoryId.eq(category_id))
    }

    pub fn find_by_article_id(article_id: i32) -> Select<Entity> {
        Self::find().filter(Column::ArticleId.eq(article_id))
    }

    pub fn delete_by_category_id(category_id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::CategoryId.eq(category_id))
    }

    pub fn delete_by_article_id(article_id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::ArticleId.eq(article_id))
    }
}
