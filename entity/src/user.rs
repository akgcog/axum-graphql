use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub e_mail: String,
    pub profile: String,
    pub thanks_to_id: i32,
    pub thanks_from_id: i32,
    pub followed_id: i32,
    pub followers_id: i32,
    #[sea_orm(nullable)]
    pub img_url: String,
    // pub creatd_at: TimeDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::thanks::Entity",
        from = "Column::ThanksToId",
        to = "super::thanks::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ThanksTo,

    #[sea_orm(
        belongs_to = "super::thanks::Entity",
        from = "Column::ThanksFromId",
        to = "super::thanks::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ThanksFrom,

    #[sea_orm(
        belongs_to = "super::follows::Entity",
        from = "Column::FollowedId",
        to = "super::follows::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Followed,

    #[sea_orm(
        belongs_to = "super::follows::Entity",
        from = "Column::FollowersId",
        to = "super::follows::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Followers
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         panic!("No RelationDef")
//     }
// }

impl Related<super::thanks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThanksTo.def()
    }
}

// impl Related<super::thanks::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::ThanksFrom.def()
//     }
// }

impl Related<super::follows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Followed.def()
        // Relation::Followers.def();
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
