use std::collections::HashMap;

use entity::repository;
use crate::{context, redirect_for_required_password_change};
use crate::utils::auth::{require_current_user, require_admin};
use crate::utils::routes::build_redirect;
use crate::utils::template::render_html_template;
use crate::error::YarmsResult;
use crate::model::create_repositories::{CreateRepoTypeFormModel, CreateRepoLayoutFormModel, CreateRepoFormModel};

use sea_orm::{entity::*, DatabaseConnection};
use actix_identity::Identity;
use actix_web::{web, HttpResponse, get, post};
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

#[post("/admin/repositories/create")]
async fn admin_post_create_repository(
    identity: Identity,
    data: web::Form<HashMap<String, String>>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    let data_str = serde_json::to_string(&data)?;

    let create_repo = serde_json::from_str::<CreateRepoFormModel>(&data_str)?;
    let create_repo_layout = serde_json::from_str::<CreateRepoLayoutFormModel>(&data_str)?;
    let create_repo_type = serde_json::from_str::<CreateRepoTypeFormModel>(&data_str)?;

    let now = chrono::Utc::now();
    let repo = repository::ActiveModel {
        name: Set(create_repo.name),
        repo_layout: Set(create_repo_layout.into()),
        repo_type: Set(create_repo_type.into()),
        allow_anonymous_reads: Set(create_repo.allow_anonymous_reads == "on"),
        allow_anonymous_writes: Set(create_repo.allow_anonymous_writes == "on"),
        created_at: Set(now),
        updated_at: Set(now),
    };

    repository::Entity::insert(repo)
        .exec(database_connection.get_ref())
        .await?;

    Ok(build_redirect("/admin/repositories/create"))
}

#[get("/admin/repositories/manage/{repo}")]
async fn admin_manage_repository(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    let context = context!(
        "user" => &current_user,
        "repository" => "poggies",
    );

    render_html_template(&template_engine, "admin/repositories/manage.html.j2", &context)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_repositories);
    cfg.service(admin_create_repository);
    cfg.service(admin_post_create_repository);
    cfg.service(admin_manage_repository);
}
