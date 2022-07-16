pub mod controller;
pub mod utils;
pub mod error;

use crate::controller::{browse, login, account, admin};
use crate::utils::template::TEMPLATE_ENGINE;

use log::info;
use migration::{Migrator, MigratorTrait};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::cookie::Key;
use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Loading environment variables
    dotenv::dotenv().ok();
    let debug = env::var("DEBUG").map(|var| var.to_lowercase() == "true").unwrap_or(false);
    let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let database_url = env::var("DATABASE_URL").expect("Required environment variable 'DATABASE_URL' not set");
    let secret_key = env::var("SECRET_KEY").expect("Required environment variable 'SECRET_KEY' not set");

    // Initilizing our logger
    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("Binding YARMS to {}:{}. Debug: {}", &host, &port, &debug);

    // Creating a database connection
    let mut database_options = ConnectOptions::new(database_url);
    database_options.max_connections(20);
    database_options.min_connections(5);
    database_options.sqlx_logging(false);
    let database_connection: DatabaseConnection = Database::connect(database_options).await?;

    // Applying any pending migrations
    Migrator::up(&database_connection, None).await?;

    // Building our message flashing framework
    let flash_store = CookieMessageStore::builder(Key::from(&secret_key.as_bytes())).build();
    let flash_framework = FlashMessagesFramework::builder(flash_store).build();

    // Starting our web server :D 
    HttpServer::new(move || {
        let identity_policy = CookieIdentityPolicy::new(&secret_key.as_bytes()).name("auth").path("/").secure(false);

        App::new()
            .app_data(web::Data::new(database_connection.clone()))
            .app_data(web::Data::new(TEMPLATE_ENGINE.clone()))
            .wrap(flash_framework.clone())
            .wrap(IdentityService::new(identity_policy))
            .wrap(Logger::default())
            .service(Files::new("/assets", "./assets/dist"))
            .configure(account::init)
            .configure(admin::init)
            .configure(browse::init)
            .configure(login::init)
    })
        .bind(format!("{}:{}", &host, &port))?
        .run()
        .await
        .map_err(anyhow::Error::from)
}
