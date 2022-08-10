use entity::repository;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CreateRepoFormModel {
    pub name: String,
    #[serde(default)] pub allow_anonymous_reads: String,
    #[serde(default)] pub allow_anonymous_writes: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "layout")]
pub enum CreateRepoLayoutFormModel {
    Generic,
    Maven {
        #[serde(default, rename = "layout_enforce_layout")] enforce_layout: String,
        #[serde(default, rename = "layout_allow_snapshots")] allow_snapshots: String,
        #[serde(default, rename = "layout_allow_releases")] allow_releases: String,
    }
}

impl Into<repository::RepoLayout> for CreateRepoLayoutFormModel {
    fn into(self) -> repository::RepoLayout {
        match self {
            CreateRepoLayoutFormModel::Generic => repository::RepoLayout::Generic,
            CreateRepoLayoutFormModel::Maven { 
                enforce_layout,
                allow_snapshots,
                allow_releases,
            } => repository::RepoLayout::Maven { 
                enforce_layout: enforce_layout == "on", 
                allow_snapshots: allow_snapshots == "on", 
                allow_releases: allow_releases == "on",
            },
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum CreateRepoTypeFormModel {
    Hosted,
    Group {
        #[serde(rename = "type_group_members")] group_members: String,
    },
    Proxy {
        #[serde(rename = "type_proxy_url")] proxy_url: String,
        #[serde(rename = "type_proxy_cache_ttl")] proxy_cache_ttl: String,
    },
}

impl Into<repository::RepoType> for CreateRepoTypeFormModel {
    fn into(self) -> repository::RepoType {
        match self {
            CreateRepoTypeFormModel::Hosted => repository::RepoType::Hosted,
            CreateRepoTypeFormModel::Group { group_members } => repository::RepoType::Group { 
                group_members: group_members.split(",").map(|i| String::from(i)).collect()
            },
            CreateRepoTypeFormModel::Proxy { proxy_url, proxy_cache_ttl } => repository::RepoType::Proxy { 
                proxy_url, 
                proxy_cache_ttl: proxy_cache_ttl.parse::<i32>().unwrap_or(0),
            },
        }
    }
}
