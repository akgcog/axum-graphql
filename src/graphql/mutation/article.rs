use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::article;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;
use chrono::{Local, DateTime};
use entity::db::Database;

#[derive(InputObject)]
pub struct CreateArticleInput {
    pub body: String,
    pub user_id: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Default)]
pub struct ArticleMutation;

#[Object]
impl ArticleMutation {
    pub async fn create_article(
        &self,
        ctx: &Context<'_>,
        input: CreateArticleInput,
    ) -> Result<article::Model> {
        let db = ctx.data::<Database>().unwrap();
        let created_at_str: DateTime<Local> = Local::now();

        let article = article::ActiveModel {
            body: Set(input.body),
            user_id: Set(input.user_id),
            created_at: Set(Some(created_at_str.to_string())),
            ..Default::default()
        };

        Ok(article.insert(db.get_connection()).await?)
    }

    pub async fn delete_article(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = article::Entity::delete_by_id(id)
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