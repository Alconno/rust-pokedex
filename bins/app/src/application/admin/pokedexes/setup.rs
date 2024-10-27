use crate::middleware::{Admin, Auth};

use super::domain::AdminPokedexes;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = AdminPokedexes {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));


    // Route to display paginated and filtered pokedex
    cfg.service(
        web::resource("/admin/pokedexes")
            .route(web::get().to(handle_pokedexes_paginated::<AdminPokedexes<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to delete user_pokedex and set attempt success to false
    cfg.service(
        web::resource("/admin/pokedexes/{user_pokedex_id}")
            .route(web::delete().to(handle_pokedex_delete::<AdminPokedexes<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );
    
}
