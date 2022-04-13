use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user, article, article::Relation, sea_orm::EntityTrait};

use crate::db::Database;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
        
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_users_test(&self, ctx: &Context<'_>) -> Result<Vec<(user::Model, Vec<article::Model>)>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find()
            .find_with_related(article::Relation::Article)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
        
    }
}
