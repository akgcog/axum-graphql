use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::follows;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateFollowedInput {
    pub followed_id: i32,
    pub follower_id: i32,
}

#[derive(Default)]
pub struct FollowedMutation;

#[Object]
impl FollowedMutation {
    pub async fn create_followed(
        &self,
        ctx: &Context<'_>,
        input: CreateFollowedInput,
    ) -> Result<follows::Model> {
        let db = ctx.data::<Database>().unwrap();

        let followed = follows::ActiveModel {
            followed_id: Set(input.followed_id),
            follower_id: Set(input.follower_id),
            ..Default::default()
        };

        Ok(followed.insert(db.get_connection()).await?)
    }

    pub async fn delete_followed(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = follows::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}