use std::future::{ready, Ready};

use actix_web::{dev::{Transform, ServiceRequest, Service, ServiceResponse, forward_ready}, Error, HttpMessage, HttpRequest, FromRequest, Result};
use futures::future::LocalBoxFuture;
use tera::Context;

#[derive(Clone)]
pub struct CarriedContext {
    pub context: Context,
}

impl FromRequest for CarriedContext {
    type Error = Error;
    type Future = Ready<Result<CarriedContext, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let carried_context = req.extensions().get::<CarriedContext>().expect("Carried context middleware not installed.").clone();
        ready(Ok(carried_context))
    }
}

// Middleware
pub struct CarriedContextService();

impl Default for CarriedContextService {
    fn default() -> Self {
        Self()
    }
}

impl<S, B> Transform<S, ServiceRequest> for CarriedContextService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CarriedContextMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CarriedContextMiddleware { service }))
    }
}

pub struct CarriedContextMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CarriedContextMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let carried_context = CarriedContext {
            context: Context::new()
        };

        req.extensions_mut().insert::<CarriedContext>(carried_context);

        let future = self.service.call(req);

        Box::pin(async move {
            Ok(future.await?)
        })
    }
}
