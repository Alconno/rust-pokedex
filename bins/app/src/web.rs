use std::sync::Arc;
use actix_web::{App, HttpServer};
use infrastructure::state::AppState;
use infrastructure::db::Pg;
use crate::middleware::RateLimiter;
use infrastructure::db::Rd;

/// Start the server
#[cfg(not(tarpaulin_include))]
pub async fn start_server(state: infrastructure::state::AppState, pg: Arc<Pg>, rd: Arc<Rd>) -> std::io::Result<()> {
    let address = format!(
        "{0}:{1}",
        config::get_default("LISTEN_ADDRESS", "127.0.0.1"),
        config::get_default("LISTEN_PORT", "4554")
    );


    HttpServer::new(move || {
        App::new()
            .wrap(RateLimiter::new()
                .with_interval(std::time::Duration::from_secs(60))
                .with_max_requests(120))
            .app_data::<AppState>(state.clone())
            .configure(|cfg| crate::application::configure(
                Arc::clone(&pg),
                Arc::clone(&rd),
                cfg
            ))
    })
    .workers(10) 
    .bind(address)
    .expect("Unable to bind server")
    .run()
    .await
}