use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::thanks;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;
use chrono::{Local, DateTime};
use entity::db::Database;

#[derive(InputObject)]
pub struct CreateThanksInput {
    pub to_thanks_id: i32,
    pub from_thanks_id: i32,
    pub comment: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Default)]
pub struct ThanksMutation;

#[Object]
impl ThanksMutation {
    pub async fn create_thanks(
        &self,
        ctx: &Context<'_>,
        input: CreateThanksInput,
    ) -> Result<thanks::Model> {
        let db = ctx.data::<Database>().unwrap();
        let created_at_str: DateTime<Local> = Local::now();

        let thanks = thanks::ActiveModel {
            to_thanks_id: Set(input.to_thanks_id),
            from_thanks_id: Set(input.from_thanks_id),
            comment: Set(input.comment),
            created_at: Set(created_at_str.to_string()),
            ..Default::default()
        };

        Ok(thanks.insert(db.get_connection()).await?)
    }

    pub async fn delete_thanks(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = thanks::Entity::delete_by_id(id)
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