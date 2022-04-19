use async_graphql::{Context, Object, Result};
use entity::{async_graphql, follow, sea_orm::EntityTrait};
use entity::db::Database;

#[derive(Default)]
pub struct FollowQuery;

#[Object]
impl FollowQuery {
    async fn get_followed(&self, ctx: &Context<'_>) -> Result<Vec<follow::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(follow::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_followed_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<follow::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(follow::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
