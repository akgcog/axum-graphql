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
    pub to_thanks_id: i32,
    pub from_thanks_id: i32,
    pub comment: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter ,DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ToThanksId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Tothanks,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FromThanksId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Fromthanks,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tothanks.def();
        Relation::Fromthanks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
  pub fn find_by_id(id: i32) -> Select<Entity> {
      Self::find().filter(Column::Id.eq(id))
  }

  pub fn find_by_tothanks_id(id: i32) -> Select<Entity> {
    Self::find().filter(Column::ToThanksId.eq(id))
  }

  pub fn find_by_fromthanks_id(id: i32) -> Select<Entity> {
    Self::find().filter(Column::FromThanksId.eq(id))
  }

  pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
      Self::delete_many().filter(Column::Id.eq(id))
  }
}
