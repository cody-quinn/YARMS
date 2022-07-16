use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
#[derive(Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub is_admin: bool,
    pub require_password_change: bool,
    pub account_locked: bool,

    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    #[sea_orm(nullable)]
    pub last_logged_in_at: Option<ChronoDateTimeUtc>,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::repository::Entity> for Entity {
    fn to() -> RelationDef {
        super::repository_access_user::Relation::Repository.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::repository_access_user::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
