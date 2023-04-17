use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "articles")]
#[graphql(concrete(name = "StdArticle", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub date: DateTime,
    pub preview: String,
    pub text: String,
    pub image_url: String,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}


impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        super::category_article::Relation::Category.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::category_article::Relation::Article.def().rev())
    }
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
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

    pub fn find_by_title(search: &str) -> Select<Entity> {
        let search_pattern = format!("%{}%", search);
        Self::find().filter(Expr::col(Column::Title).like(search_pattern))
    }


    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }

    pub fn find_by_user_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(id))
    }

}
