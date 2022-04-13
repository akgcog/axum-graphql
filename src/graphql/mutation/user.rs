use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::DeleteResult;

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub e_mail: String,
    pub password: String,
    pub profile: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();

        let user = user::ActiveModel {
            name: Set(input.name),
            e_mail: Set(input.e_mail),
            password: Set(input.password),
            profile: Set(input.profile),
            ..Default::default()
        };

        Ok(user.insert(db.get_connection()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = user::Entity::delete_by_id(id)
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