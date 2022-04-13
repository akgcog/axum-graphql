use async_graphql::{Context, Object, Result};
use entity::{async_graphql, follows, sea_orm::EntityTrait};

use crate::db::Database;

#[derive(Default)]
pub struct FollowedQuery;

#[Object]
impl FollowedQuery {
    async fn get_followed(&self, ctx: &Context<'_>) -> Result<Vec<follows::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(follows::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_followed_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<follows::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(follows::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
