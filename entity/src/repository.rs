use sea_orm::{entity::prelude::*, sea_query::{ValueType, ValueTypeErr}, TryGetable};
use serde::{Serialize, Deserialize};

use crate::prep_json_field;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "repository")]
#[derive(Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub name: String,
    pub repo_layout: RepoLayout,
    pub repo_type: RepoType,
    pub allow_anonymous_reads: bool,
    pub allow_anonymous_writes: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum RepoLayout {
    Generic,
    Maven {
        enforce_layout: bool,
        allow_snapshots: bool,
        allow_releases: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum RepoType {
    Hosted,
    Group {
        group_members: Vec<String>,
    },
    Proxy {
        proxy_url: String,
        proxy_cache_ttl: i32,
    },
}

prep_json_field!(RepoLayout);
prep_json_field!(RepoType);

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::repository_access_user::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::repository_access_user::Relation::Repository.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
