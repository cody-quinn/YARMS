use std::env;

use lazy_static::lazy_static;
use actix_web::HttpResponse;
use tera::Tera;

use crate::error::YarmsResult;

lazy_static! {
    pub static ref TEMPLATE_ENGINE: Tera = match Tera::new(&env::var("TEMPLATE_DIR").unwrap_or_else(|_| String::from("templates/**/*.html.j2"))) {
        Err(_) => { panic!("Failed to initilize Tera"); },
        Ok(t) => t,
    };
}

pub fn render_html_template(template_engine: &tera::Tera, template_name: &str, context: &tera::Context) -> YarmsResult<HttpResponse> {
    let body = template_engine.render(template_name, context)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[macro_export]
macro_rules! context {
    ( $( $x:expr => $y:expr ),* $(,)? ) => {
        {
            #[allow(unused_mut)]
            let mut context = tera::Context::new();
            $(
                context.insert($x, $y);
            )*
            context
        }
    };
}
