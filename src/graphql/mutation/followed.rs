use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::followed;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateFollowedInput {
    pub to_user_id: i32,
    pub from_user_id: i32,
}

#[derive(Default)]
pub struct FollowedMutation;

#[Object]
impl FollowedMutation {
    pub async fn create_followed(
        &self,
        ctx: &Context<'_>,
        input: CreateFollowedInput,
    ) -> Result<followed::Model> {
        let db = ctx.data::<Database>().unwrap();

        let followed = followed::ActiveModel {
            to_user_id: Set(input.to_user_id),
            from_user_id: Set(input.from_user_id),
            ..Default::default()
        };

        Ok(followed.insert(db.get_connection()).await?)
    }

    pub async fn delete_followed(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = followed::Entity::delete_by_id(id)
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