use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "follows")]
#[graphql(concrete(name = "Follows", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub followed_id: i32,
    pub follower_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        // panic!("No RelationDef")
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::FollowedId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
  pub fn find_by_id(id: i32) -> Select<Entity> {
      Self::find().filter(Column::Id.eq(id))
  }

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}
