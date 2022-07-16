use sea_orm::entity::*;
use actix_identity::Identity;
use sea_orm::DatabaseConnection;

use entity::user;
use crate::error::{YarmsResult, YarmsError};

pub async fn require_current_user(
    identity: &Identity,
    database_connection: &DatabaseConnection,
) -> YarmsResult<user::Model> {
    match optional_current_user(identity, database_connection).await? {
        Some(user) => Ok(user),
        None => Err(YarmsError::Unauthorized),
    }
}

pub async fn optional_current_user(
    identity: &Identity,
    database_connection: &DatabaseConnection,
) -> YarmsResult<Option<user::Model>> {
    match identity.identity() {
        Some(identity) => Ok(user::Entity::find_by_id(identity.parse()?).one(database_connection).await?),
        None => Ok(None)
    }
}

pub async fn require_admin(
    user: &user::Model,
) -> YarmsResult<&user::Model> {
    match user.is_admin {
        true => Ok(user),
        false => Err(YarmsError::Forbidden),
    }
}

pub fn validate_password(
    password: &str,
) -> std::result::Result<(), &str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters in length");
    }

    Ok(())
}

#[macro_export]
macro_rules! redirect_for_required_password_change {
    ($user:expr) => {
        if $user.require_password_change {
            return Ok(build_redirect("/account/change_password"));
        }
    };
}
