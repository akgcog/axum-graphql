use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};
use crate::article::Model as ArticleModel;
use crate::article::Entity as ArticleEntity;
use crate::follow::Model as FollowModel;
use crate::follow::Entity as FollowEntity;
use crate::thanks::Model as ThanksModel;
use crate::thanks::Entity as ThanksEntity;

use crate::db::Database;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()), complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub e_mail: String,
    pub password: String,
    pub profile: Option<String>,
    pub img_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[ComplexObject]
impl Model {
    async fn article(&self, ctx: &Context<'_>) -> Result<Vec<ArticleModel>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(ArticleEntity::find_by_user_id(self.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn followed(&self, ctx: &Context<'_>) -> Result<Vec<FollowModel>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(FollowEntity::find_by_followed_id(self.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn follower(&self, ctx: &Context<'_>) -> Result<Vec<FollowModel>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(FollowEntity::find_by_follower_id(self.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn tothanks(&self, ctx: &Context<'_>) -> Result<Vec<ThanksModel>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(ThanksEntity::find_by_tothanks_id(self.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn fromthanks(&self, ctx: &Context<'_>) -> Result<Vec<ThanksModel>> {
        let db = ctx.data::<Database>().unwrap();
        Ok(ThanksEntity::find_by_fromthanks_id(self.id)
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::article::Entity")]
    Article,
    #[sea_orm(has_many = "super::follow::Entity")]
    Follow,
    #[sea_orm(has_many = "super::thanks::Entity")]
    Thanks,
}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::follow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Follow.def()
    }
}

impl Related<super::thanks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Thanks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
  pub fn find_by_id(id: i32) -> Select<Entity> {
      Self::find().filter(Column::Id.eq(id))
  }

  pub fn find_by_name(name: &str) -> Select<Entity> {
      Self::find().filter(Column::Name.eq(name))
  }

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}


