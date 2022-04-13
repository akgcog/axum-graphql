use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "article")]
#[graphql(concrete(name = "Article", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub body: String,
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
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

  pub fn find_by_user_id(id: i32) -> Select<Entity> {
    Self::find().filter(Column::UserId.eq(id))
}

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}
