use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use error::Error as AdminError;
use futures::future;
use futures_util::future::{LocalBoxFuture, Ready};
use std::{
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct Admin;

impl<S> Transform<S, ServiceRequest> for Admin
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AdminMiddleware {
            service: Arc::new(Mutex::new(service)),
        })
    }
}

// Middleware struct to handle admin authentication
pub struct AdminMiddleware<S> {
    service: Arc<Mutex<S>>,
}

impl<S> Service<ServiceRequest> for AdminMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let service = self.service.lock().unwrap();
        service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user_role = req.extensions().get::<(&str, String)>().map(|(_, role)| role.clone());

        // Check if the user has the Admin role
        if let Some(role) = user_role {
            if role == "Admin" {
                let fut = self.service.lock().unwrap().call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        // If user doesn't have admin role, return Unauthorized response
        let response = ServiceResponse::new(
            req.into_parts().0,
            AdminError::Unauthorized("You are not authorized to access this resource".to_owned())
                .error_response(),
        );
        Box::pin(async move { Ok(response) })
    }
}