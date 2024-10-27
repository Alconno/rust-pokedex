use crate::middleware::{Admin, Auth};

use super::domain::AdminUsers;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = AdminUsers {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));

    // Route to display paginated and filtered users
    cfg.service(
        web::resource("/admin/users")
            .route(web::get().to(handle_users_paginated::<AdminUsers<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to show an user
    cfg.service(
        web::resource("/admin/users/{user_id}")
            .route(web::get().to(handle_user_show::<AdminUsers<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth)
    );


    // Route to logout an user
    cfg.service(
        web::resource("/admin/users/{user_id}/logout")
            .route(web::patch().to(handle_user_logout::<AdminUsers<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to deactivate (soft delete) an user
    cfg.service(
        web::resource("/admin/users/{user_id}/deactivate")
            .route(web::patch().to(handle_user_deactivation::<AdminUsers<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to activate an user
    cfg.service(
        web::resource("/admin/users/{user_id}/activate")
            .route(web::patch().to(handle_user_activation::<AdminUsers<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );
    
}
