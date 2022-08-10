use sea_orm::{entity::*, DatabaseConnection};
use actix_web::{HttpResponse, web, get};
use actix_identity::Identity;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct RepoIndexPath {
    repository: String,
}

#[get("/repo/{repository}")]
async fn repo_index(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
    path: web::Path<RepoIndexPath>,
) -> YarmsResult<HttpResponse> {
    let context = context!();

    let current_user = optional_current_user(&identity, &database_connection).await?;
    if let Some(current_user) = &current_user {
        redirect_for_required_password_change!(&current_user);
    }

    render_html_template(&template_engine, "index.html.j2", &context)
}

#[derive(Deserialize)]
struct RepoGetPath {
    repository: String,
    path: String,
}

#[get("/repo/{repository}/{path}")]
async fn repo_get(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
    path: web::Path<RepoGetPath>,
) -> YarmsResult<HttpResponse> {
    let context = context!();

    let current_user = optional_current_user(&identity, &database_connection).await?;
    if let Some(current_user) = &current_user {
        redirect_for_required_password_change!(&current_user);
    }

    render_html_template(&template_engine, "index.html.j2", &context)
}

// #[put("/repo/{repository}/{path}")]
// #[delete("/repo/{repository}/{path}")]

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
