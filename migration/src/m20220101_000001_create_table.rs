// use sea_schema::migration::prelude::*;
use entity::{
    user,
    thanks,
    follows,
    article,
    sea_orm::{DbBackend, EntityTrait, Schema},
};
use sea_schema::migration::{
    sea_query::*,
    *,
};

pub struct Migration;

fn get_seaorm_create_stmt<E: EntityTrait>(e: E) -> TableCreateStatement {
    let schema = Schema::new(DbBackend::Postgres);

    schema
        .create_table_from_entity(e)
        .if_not_exists()
        .to_owned()
}

fn get_seaorm_drop_stmt<E: EntityTrait>(e: E) -> TableDropStatement {
    Table::drop().table(e).if_exists().to_owned()
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmts = vec![
                        get_seaorm_create_stmt(user::Entity),
                        get_seaorm_create_stmt(article::Entity),
                        // get_seaorm_create_stmt(thanks::Entity),
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
                        // get_seaorm_drop_stmt(thanks::Entity),
                        ];

        for stmt in stmts {
            manager.drop_table(stmt.to_owned()).await?;
        }

        Ok(())
    }

}


