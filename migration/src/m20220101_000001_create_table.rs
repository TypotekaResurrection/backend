use entity::{sea_orm::{DbBackend, EntityTrait, Schema}, article, user, comment, category, category_article};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![
            get_seaorm_create_stmt(user::Entity),
            get_seaorm_create_stmt(article::Entity),
            get_seaorm_create_stmt(comment::Entity),
            get_seaorm_create_stmt(category::Entity),
            get_seaorm_create_stmt(category_article::Entity)
        ];

        for stmt in stmts {
            manager.create_table(stmt.to_owned()).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![
            get_seaorm_drop_stmt(article::Entity),
            get_seaorm_drop_stmt(user::Entity),
            get_seaorm_drop_stmt(comment::Entity),
            get_seaorm_drop_stmt(category::Entity),
            get_seaorm_drop_stmt(category_article::Entity)
        ];

        for stmt in stmts {
            manager.drop_table(stmt.to_owned()).await?;
        }

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

fn get_seaorm_create_stmt<E: EntityTrait>(e: E) -> TableCreateStatement {
    let schema = Schema::new(DbBackend::Sqlite);

    schema
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}

fn get_seaorm_drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}
