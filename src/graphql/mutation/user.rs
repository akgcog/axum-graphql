use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user;
use entity::sea_orm::{ActiveModelTrait, Set};
use crate::graphql::mutation::{DeleteResult};
use chrono::{Local, DateTime};
use entity::db::Database;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub e_mail: String,
    pub password: String,
    pub profile: Option<String>,
    pub img_url: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub e_mail: Option<String>,
    pub password: Option<String>,
    pub profile: Option<String>,
    pub img_url: Option<String>,
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
        let created_at_str: DateTime<Local> = Local::now();

        let user = user::ActiveModel {
            name: Set(input.name),
            e_mail: Set(input.e_mail),
            password: Set(input.password),
            profile: Set(input.profile),
            img_url: Set(input.img_url),
            created_at: Set(Some(created_at_str.to_string())),
            ..Default::default()
        };

        Ok(user.insert(db.get_connection()).await?)
    }

    pub async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let updated_at_str: DateTime<Local> = Local::now();
        let user: Option<user::Model> = user::Entity::find_by_id(id).one(db.get_connection()).await?;
        let mut user: user::ActiveModel = user.unwrap().into();
        // let mut user = user::ActiveModel {
        //     name: Set(input.name),
        //     e_mail: Set(input.e_mail),
        //     password: Set(input.password),
        //     profile: Set(input.profile),
        //     img_url: Set(input.img_url),
        //     updated_at: Set(Some(updated_at_str.to_string())),
        // }.unwrap().into();

        user.name = Set(input.name);
        user.e_mail = Set(input.e_mail);
        user.password = Set(input.password);
        user.profile = Set(input.profile);
        user.img_url = Set(input.img_url);
        user.updated_at = Set(Some(updated_at_str.to_string()));

        Ok(user.update(db.get_connection()).await?)
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