use async_graphql::{Context, Object, Result};
use entity::{async_graphql, user, sea_orm::EntityTrait};

use crate::db::Database;

pub struct Test {
    pub id: i32,
    pub name: String,
    pub e_mail: String,
    pub profile: String,
    pub thx_to: i32,
    pub thx_from: i32,
    pub follows: i32,
    pub followers: i32,
    pub img_url: String,
    pub a: i32
    // pub creatd_at: NaiveDateTime,
    // pub update_at: TimeDateTime,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<Test>> {
        let db = ctx.data::<Database>().unwrap();
        let a = 3;

        Ok(user::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?);
        println!("{}", a)
        
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<user::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}
