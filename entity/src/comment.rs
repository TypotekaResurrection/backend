use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany, Value, DeriveRelation};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub date: DateTime,
    pub content: String,
    pub article_id: i32,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::article::Entity",
        from = "Column::ArticleId",
        to = "super::article::Column::Id",
        on_delete = "Cascade"
    )]
    Article,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}


impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
    pub fn find_by_article_id(article_id: i32) -> Select<Entity> {
        Self::find().filter(Column::ArticleId.eq(article_id))
    }
    pub fn delete_by_article_id(article_id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::ArticleId.eq(article_id))
    }
    pub fn find_by_user_id(user_id: i32) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(user_id))
    }
}
