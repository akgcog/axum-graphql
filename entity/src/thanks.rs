use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "thanks")]
#[graphql(concrete(name = "Thanks", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub to_user_id: i32,
    pub from_user_id: i32,
    pub comment: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // #[sea_orm(has_many = "super::user::Entity")]
    // User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

// impl Related<super::user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
  pub fn find_by_id(id: i32) -> Select<Entity> {
      Self::find().filter(Column::Id.eq(id))
  }

//   pub fn find_by_name(name: &str) -> Select<Entity> {
//       Self::find().filter(Column::Name.eq(name))
//   }

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}
