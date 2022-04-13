use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::article;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateArticleInput {
    pub body: String,
    pub user_id: Option<i32>,
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

        let article = article::ActiveModel {
            body: Set(input.body),
            user_id: Set(input.user_id),
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