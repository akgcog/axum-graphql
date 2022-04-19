use async_graphql::{Context, Object, Result};
use entity::{async_graphql, thanks, sea_orm::EntityTrait};
use entity::db::Database;

#[derive(Default)]
pub struct ThanksQuery;

#[Object]
impl ThanksQuery {
    async fn get_followed(&self, ctx: &Context<'_>) -> Result<Vec<thanks::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(thanks::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_thanks_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<thanks::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(thanks::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
