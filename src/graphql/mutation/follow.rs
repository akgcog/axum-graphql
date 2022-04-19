use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::follow;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;
use chrono::{Local, DateTime};
use entity::db::Database;

#[derive(InputObject)]
pub struct CreateFollowInput {
    pub followed_id: i32,
    pub follower_id: i32,
    pub created_at: Option<String>,
}

#[derive(Default)]
pub struct FollowMutation;

#[Object]
impl FollowMutation {
    pub async fn create_follow(
        &self,
        ctx: &Context<'_>,
        input: CreateFollowInput,
    ) -> Result<follow::Model> {
        let db = ctx.data::<Database>().unwrap();
        let created_at_str: DateTime<Local> = Local::now();

        let follow = follow::ActiveModel {
            followed_id: Set(input.followed_id),
            follower_id: Set(input.follower_id),
            created_at: Set(created_at_str.to_string()),
            ..Default::default()
        };

        Ok(follow.insert(db.get_connection()).await?)
    }

    pub async fn delete_follow(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = follow::Entity::delete_by_id(id)
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