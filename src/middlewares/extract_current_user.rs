use std::rc::Rc;

use actix_identity::Identity;
use actix_web::{FromRequest, Error, HttpRequest, HttpMessage, dev::{ServiceResponse, ServiceRequest, Transform, Service, forward_ready}};
use entity::user;
use futures::future::{Ready, ready, LocalBoxFuture};
use sea_orm::DatabaseConnection;

// use crate::utils::get_current_user;

#[derive(Clone)]
pub struct CurrentUser {
    pub user: Option<user::Model>,
}

impl FromRequest for CurrentUser {
    type Error = Error;
    type Future = Ready<Result<CurrentUser, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let carried_context = req.extensions().get::<CurrentUser>().expect("Inject current user middleware not installed.").clone();
        ready(Ok(carried_context))
    }
}

// Middleware
pub struct ExtractCurrentUserService {
    database_connection: Rc<DatabaseConnection>,
}

impl ExtractCurrentUserService {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection: Rc::new(database_connection),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ExtractCurrentUserService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ExtractCurrentUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExtractCurrentUserMiddleware {
            database_connection: Rc::clone(&self.database_connection), 
            service: Rc::new(service),
        }))
    }
}

pub struct ExtractCurrentUserMiddleware<S> {
    database_connection: Rc<DatabaseConnection>,
    service: Rc<S>,
}

impl<S> Clone for ExtractCurrentUserMiddleware<S> {
    fn clone(&self) -> Self {
        Self { 
            database_connection: Rc::clone(&self.database_connection),
            service: Rc::clone(&self.service), 
        }
    }
}

impl<S, B> Service<ServiceRequest> for ExtractCurrentUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service: Rc<S> = Rc::clone(&self.service);

        Box::pin(async move {
            let _identity = req.extract::<Identity>().await;
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
