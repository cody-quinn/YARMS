use sea_orm::{entity::*, DatabaseConnection};
use actix_web::{HttpResponse, web, get};
use actix_identity::Identity;
use tera::Tera;

use entity::repository;
use crate::{context, redirect_for_required_password_change};
use crate::error::YarmsResult;
use crate::utils::template::render_html_template;
use crate::utils::auth::optional_current_user;
use crate::utils::routes::build_redirect;

#[get("/")]
async fn index(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = optional_current_user(&identity, &database_connection).await?;

    // TODO: Make this a middleware! 
    if let Some(current_user) = &current_user {
        redirect_for_required_password_change!(&current_user);
    }

    let repositories: Vec<repository::Model> = repository::Entity::find().all(database_connection.get_ref()).await?;

    let context = context!(
        "user" => &current_user,
        "repositories" => &repositories,
    );

    render_html_template(&template_engine, "index.html.j2", &context)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
