use super::domain::Login;
use super::infrastructure::{PgService, PgRepository};
use actix_web::web;
use super::http::*;
use infrastructure::db::Pg;
use std::sync::Arc;



pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    // Create a Login instance with the PgService and PgRepository
    let service = Login {
        service: PgService { pg_pool: pg_pool.clone() },
        repository: PgRepository { pg_pool }
    };

    // Share the Login service across Actix web app
    cfg.app_data(web::Data::new(service));

    // Route for handling login page load
    cfg.route(
        "/auth/index",
        web::get().to(index)
    );

    // Route for handling login requests
    cfg.route(
        "/auth/login",
        web::post().to(handle_login::<Login<PgRepository, PgService>>)
    );

    // Route for handling token refresh requests
    cfg.route(
        "/auth/refresh",
        web::post().to(handle_refresh::<Login<PgRepository, PgService>>)
    );

    // Route for handling logout requests
    cfg.route(
        "/auth/logout",
        web::post().to(handle_logout::<Login<PgRepository, PgService>>)
    );
}
