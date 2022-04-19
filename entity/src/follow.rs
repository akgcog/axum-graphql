use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "follow")]
#[graphql(concrete(name = "Follow", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub followed_id: i32,
    pub follower_id: i32,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter ,DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FollowedId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Followed,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FollowerId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Follower,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Followed.def();
        Relation::Follower.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
  pub fn find_by_id(id: i32) -> Select<Entity> {
      Self::find().filter(Column::Id.eq(id))
  }

  pub fn find_by_followed_id(id: i32) -> Select<Entity> {
    Self::find().filter(Column::FollowedId.eq(id))
  }

  pub fn find_by_follower_id(id: i32) -> Select<Entity> {
    Self::find().filter(Column::FollowerId.eq(id))
  }

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}
