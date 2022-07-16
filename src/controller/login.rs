use actix_web_flash_messages::{IncomingFlashMessages, FlashMessage};
use sea_orm::{entity::*, query::*, DatabaseConnection};
use actix_web::{HttpResponse, web, get, post};
use actix_identity::Identity;
use serde::Deserialize;
use tera::Tera;

use entity::user;
use security::check_password;

use crate::utils::flashing::extract_flash_message;
use crate::{unwrap_some_else_return, context};
use crate::error::YarmsResult;
use crate::utils::template::render_html_template;
use crate::utils::routes::build_redirect;

#[get("/login")]
async fn login(
    flashed_messages: IncomingFlashMessages,
    identity: Identity,
    template_engine: web::Data<Tera>,
) -> YarmsResult<HttpResponse> {
    if let Some(_) = identity.identity() {
        return Ok(build_redirect("/"));
    }

    let context = context!("flash_message" => &extract_flash_message(&flashed_messages));
    render_html_template(&template_engine, "login.html.j2", &context)
}

#[derive(Deserialize)]
struct LoginFormData {
    username: String,
    password: String,
}

#[post("/login")]
async fn post_login(
    identity: Identity,
    form: web::Form<LoginFormData>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    if let Some(_) = identity.identity() {
        return Ok(build_redirect("/"));
    }

    let database_user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Username.contains(&form.username))
        .one(database_connection.get_ref())
        .await?;

    let database_user = unwrap_some_else_return!(database_user, {
        FlashMessage::info("Invalid username and password").send();
        Ok(build_redirect("/login"))
    });

    if !check_password(&form.password, &database_user.password) {
        FlashMessage::info("Invalid username and password").send();
        return Ok(build_redirect("/login"));
    }

    identity.remember(database_user.id.to_string());
    if database_user.require_password_change {
        Ok(build_redirect("/account/change_password"))
    } else {
        Ok(build_redirect("/"))
    }
}

#[post("/logout")]
async fn logout(identity: Identity) -> YarmsResult<HttpResponse> {
    identity.forget();
    Ok(build_redirect("/"))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(post_login);
    cfg.service(logout);
}
