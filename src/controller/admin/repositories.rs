use std::collections::HashMap;

use entity::repository;
use crate::{context, redirect_for_required_password_change};
use crate::utils::auth::{require_current_user, require_admin};
use crate::utils::routes::build_redirect;
use crate::utils::template::render_html_template;
use crate::error::YarmsResult;

use sea_orm::{entity::*, DatabaseConnection};
use actix_identity::Identity;
use actix_web::{web, HttpResponse, get, post};
use serde::Deserialize;
use tera::Tera;

#[get("/admin/repositories")]
async fn admin_repositories(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    // TODO: Make this a middleware! 
    redirect_for_required_password_change!(current_user);

    let repositories: Vec<repository::Model> = repository::Entity::find().all(database_connection.get_ref()).await?;
    
    let context = context!(
        "user" => &current_user,
        "repositories" => &repositories,
    );

    render_html_template(&template_engine, "admin/repositories/repositories.html.j2", &context)
}

#[get("/admin/repositories/create")]
async fn admin_create_repository(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    // TODO: Make this a middleware! 
    redirect_for_required_password_change!(current_user);

    let context = context!("user" => &current_user);
    render_html_template(&template_engine, "admin/repositories/create.html.j2", &context)
}

#[derive(Debug, Deserialize)]
pub struct CreateRepositoryModel {
    pub name: String,
    pub repo_layout: repository::RepoLayout,
    pub repo_type: repository::RepoType,
    pub allow_anonymous_reads: bool,
    pub allow_anonymous_writes: bool,
}

#[post("/admin/repositories/create")]
async fn admin_post_create_repository(
    identity: Identity,
    data: web::Form<HashMap<String, String>>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    println!("{:?}", &data);

    Ok(build_redirect("/admin/repositories/create"))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_repositories);
    cfg.service(admin_create_repository);
    cfg.service(admin_post_create_repository);
}
