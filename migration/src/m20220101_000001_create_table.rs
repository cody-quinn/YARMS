use sea_orm_migration::{prelude::*, sea_orm::{Set, ActiveModelTrait, prelude::ChronoDateTimeUtc}};
use chrono::prelude::*;

use entity::{repository, user, repository_access_user};
use security::hash_password_rng_salt;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(sea_query::Table::create()
                .table(repository::Entity)
                .if_not_exists()
                .col(ColumnDef::new(repository::Column::Name).string().not_null().primary_key())
                .col(ColumnDef::new(repository::Column::AllowAnonymousReads).boolean().not_null().default(false))
                .col(ColumnDef::new(repository::Column::AllowAnonymousWrites).boolean().not_null().default(false))
                .col(ColumnDef::new(repository::Column::RepoLayout).json().not_null())
                .col(ColumnDef::new(repository::Column::RepoType).json().not_null())
                .col(ColumnDef::new(repository::Column::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(repository::Column::UpdatedAt).timestamp().not_null())
                .to_owned()
            )
            .await?;
        manager.create_table(sea_query::Table::create()
                .table(user::Entity)
                .if_not_exists()
                .col(ColumnDef::new(user::Column::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(user::Column::Username).string().not_null())
                .col(ColumnDef::new(user::Column::Password).string().not_null())
                .col(ColumnDef::new(user::Column::IsAdmin).boolean().not_null().default(false))
                .col(ColumnDef::new(user::Column::RequirePasswordChange).boolean().not_null().default(false))
                .col(ColumnDef::new(user::Column::AccountLocked).boolean().not_null().default(false))
                .col(ColumnDef::new(user::Column::CreatedAt).timestamp().not_null())
                .col(ColumnDef::new(user::Column::UpdatedAt).timestamp().not_null())
                .col(ColumnDef::new(user::Column::LastLoggedInAt).timestamp().default(Option::<ChronoDateTimeUtc>::None))
                .to_owned()
            )
            .await?;
        manager.create_table(sea_query::Table::create()
                .table(repository_access_user::Entity)
                .if_not_exists()
                .col(ColumnDef::new(repository_access_user::Column::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(repository_access_user::Column::RepositoryName).string().not_null())
                .col(ColumnDef::new(repository_access_user::Column::UserId).integer().not_null())
                .col(ColumnDef::new(repository_access_user::Column::CanRead).boolean().not_null())
                .col(ColumnDef::new(repository_access_user::Column::CanWrite).boolean().not_null())
                .foreign_key(ForeignKeyCreateStatement::new()
                    .name("FK_repo_user_to_repo_name")
                    .from_tbl(repository_access_user::Entity)
                    .from_col(repository_access_user::Column::RepositoryName)
                    .to_tbl(repository::Entity)
                    .to_col(repository::Column::Name),
                )
                .foreign_key(ForeignKeyCreateStatement::new()
                    .name("FK_repo_user_to_user_id")
                    .from_tbl(repository_access_user::Entity)
                    .from_col(repository_access_user::Column::UserId)
                    .to_tbl(user::Entity)
                    .to_col(user::Column::Id),
                )
                .to_owned()
            )
            .await?;

        let connection = manager.get_connection();
        let default_password_hash = hash_password_rng_salt("yams_are_yummy").unwrap();

        user::ActiveModel {
            username: Set(String::from("admin")),
            password: Set(default_password_hash.clone()),
            is_admin: Set(true),
            require_password_change: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }
        .insert(connection)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop()
                .table(repository_access_user::Entity)
                .to_owned()
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop()
                .table(repository::Entity)
                .to_owned()
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop()
                .table(user::Entity)
                .to_owned()
            )
            .await?;

        Ok(())
    }
}
