use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sea_orm::{entity::*, DatabaseConnection};
use actix_identity::Identity;
use actix_web::{web, HttpResponse, get, post};
use serde::Deserialize;
use tera::Tera;

use entity::user;
use security::hash_password_rng_salt;

use crate::utils::flashing::extract_flash_message;
use crate::{context, redirect_for_required_password_change};
use crate::error::YarmsResult;
use crate::utils::template::render_html_template;
use crate::utils::auth::{require_current_user, validate_password};
use crate::utils::routes::build_redirect;

#[get("/account")]
async fn account(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;

    // TODO: Make this a middleware! 
    redirect_for_required_password_change!(current_user);

    let context = context!("user" => &current_user);
    render_html_template(&template_engine, "account/dashboard.html.j2", &context)
}

#[get("/account/change_password")]
async fn account_change_password(
    flashed_messages: IncomingFlashMessages,
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    let context = context!("user" => &current_user, "flash_message" => &extract_flash_message(&flashed_messages));
    render_html_template(&template_engine, "account/change_password.html.j2", &context)
}

#[derive(Deserialize)]
struct ChangePasswordFormData {
    new_password: String,
    confirm_password: String,
}

#[post("/account/change_password")]
async fn post_account_change_password(
    identity: Identity,
    form: web::Form<ChangePasswordFormData>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;

    // Checking to make sure both password and confirm password are the same
    if form.new_password != form.confirm_password {
        FlashMessage::info("Both passwords must be the same").send();
        return Ok(build_redirect("/account/change_password"))
    }

    // Validating the password inputed
    if let Err(reason) = validate_password(&form.new_password) {
        FlashMessage::info(reason).send();
        return Ok(build_redirect("/account/change_password"))
    }

    // Logic to actually update the password
    let new_password_hash = hash_password_rng_salt(&form.new_password)?;

    let mut current_user_active_model: user::ActiveModel = current_user.into();
    current_user_active_model.password = Set(new_password_hash);
    current_user_active_model.require_password_change = Set(false);
    current_user_active_model.update(database_connection.get_ref()).await?;

    Ok(build_redirect("/account"))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(account);
    cfg.service(account_change_password);
    cfg.service(post_account_change_password);
}
