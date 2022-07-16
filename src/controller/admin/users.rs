use sea_orm::DatabaseConnection;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, get};
use tera::Tera;

use crate::{context, redirect_for_required_password_change};
use crate::utils::auth::{require_current_user, require_admin};
use crate::utils::routes::build_redirect;
use crate::utils::template::render_html_template;
use crate::error::YarmsResult;

#[get("/admin/users")]
async fn admin_users(
    identity: Identity,
    template_engine: web::Data<Tera>,
    database_connection: web::Data<DatabaseConnection>,
) -> YarmsResult<HttpResponse> {
    let current_user = require_current_user(&identity, &database_connection).await?;
    require_admin(&current_user).await?;

    // TODO: Make this a middleware! 
    redirect_for_required_password_change!(current_user);
    
    let context = context!("user" => &current_user);
    render_html_template(&template_engine, "admin/users/users.html.j2", &context)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_users);
}
