use std::{sync::{Arc, Mutex}, task::Poll, time::{Duration, Instant}};
use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use error::Error as LimiterError;
use dashmap::DashMap;
use futures::future::{self, LocalBoxFuture, Ready};
use actix_web::ResponseError;

#[derive(Clone)]
pub struct MemoryStorage{
    pub inner: Arc<DashMap<String, (usize, Instant)>>,
}
impl MemoryStorage {
    pub fn new() -> Self {
        debug!("Creating new MemoryStorage");
        MemoryStorage {
            inner: Arc::new(DashMap::<String, (usize, Instant)>::new()),
        }
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    pub store: MemoryStorage,
    pub max_requests: usize,
    pub interval: Duration,
    pub identifier: Arc<dyn Fn(&ServiceRequest) -> Result<String, Error> + Send + Sync>,
}


impl RateLimiter {
    pub fn new() -> Self {
        let identifier = |req: &ServiceRequest| { 
            Ok(req.peer_addr().unwrap().ip().to_string())
        };
        RateLimiter {
            store: MemoryStorage::new(),
            max_requests: 0,
            interval: std::time::Duration::from_secs(0),
            identifier: Arc::new(identifier),
        }
    }

    /// Specify the interval. The counter for a client is reset after this interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Specify the maximum number of requests allowed in the given interval.
    pub fn with_max_requests(mut self, max_requests: usize) -> Self {
        self.max_requests = max_requests;
        self
    }

    /// Function to get the identifier for the client request
    #[allow(dead_code)]
    pub fn with_identifier<F>(mut self, identifier: F) -> Self
    where
        F: Fn(&ServiceRequest) -> Result<String, Error> + Send + Sync + 'static,
    {
        self.identifier = Arc::new(identifier);
        self
    }
}

impl<S> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(RateLimiterMiddleware {
            service: Arc::new(Mutex::new(service)),
            limiter: self.store.clone(),
            max_requests: self.max_requests,
            interval: self.interval.as_secs(),
            identifier: self.identifier.clone(),
        })
    }
}

#[derive(Clone)]
pub struct RateLimiterMiddleware<S>{
    service: Arc<Mutex<S>>,
    limiter: MemoryStorage,
    max_requests: usize,
    interval: u64,
    identifier: Arc<dyn Fn(&ServiceRequest) -> Result<String, Error>+'static>,
}

impl<S> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let service = self.service.lock().unwrap();
        service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let store = self.limiter.clone();
        let srv = self.service.clone();
        let max_requests = self.max_requests;
        let interval = std::time::Duration::from_secs(self.interval);
        let identifier = self.identifier.clone();
        
        return Box::pin(async move {
            let identifier = (identifier)(&req)?;
    
            if let Some(mut entry) = store.inner.get_mut(&identifier) {
                // Identifier exists in store - check remaining value and expiry
                let (remaining, expiry) = entry.value_mut();
                if Instant::now().duration_since(*expiry) >= interval {
                    // Current limit tracking expired, reset remaining value
                    *remaining = max_requests;
                    *expiry = Instant::now();
                } else {
                    if *remaining-1 <= 0 {
                        info!("Limit exceeded for client: {}", &identifier);
                        // Create response here before moving `req`
                        let response = ServiceResponse::new(
                            req.into_parts().0,
                            LimiterError::TooManyRequests("Chill there bud.".to_owned())
                                .error_response(),
                        );
                        return Ok(response.into());
                    }
                    *remaining -= 1;
                }
    
            } else {
                // Identifier doesn't exist in store - create it
                store.inner.insert(identifier, (max_requests, Instant::now()));
            }
            
            let fut = srv.lock().unwrap().call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
    
}
