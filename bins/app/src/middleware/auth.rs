use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use error::Error as AuthError;
use futures::future;
use futures_util::future::{LocalBoxFuture, Ready};
use support::helpers::{http::extract_token_by_name, verify_access_token::verify_access_token};
use std::{
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

#[derive(Clone)]
pub struct Auth;

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthMiddleware {
            service: Arc::new(Mutex::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Arc<Mutex<S>>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
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

        let access_token = extract_token_by_name(&req, "access_token");
        // Verify access token
        let (auth_user_id, auth_user_role) = match verify_access_token(&access_token) {
            Ok(user) => user,
            Err(AuthError::Unauthorized(_)) => {
                let response = ServiceResponse::new(
                    req.into_parts().0,
                    AuthError::Unauthorized("Invalid access token".to_owned())
                        .error_response(),
                );
                return Box::pin(async move { Ok(response) });
            }
            Err(other_error) => {
                let response = ServiceResponse::new(
                    req.into_parts().0,
                    AuthError::InternalError(other_error.to_string())
                        .error_response(),
                );
                return Box::pin(async move { Ok(response) });
            }
        };

        // Insert authenticated user ID into request extensions
        req.extensions_mut().insert(auth_user_id);

        // Insert authenticated user role into request extensions
        req.extensions_mut().insert(("role", auth_user_role));

        let fut = self.service.lock().unwrap().call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}