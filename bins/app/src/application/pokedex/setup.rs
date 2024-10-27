use crate::middleware::Auth;

use super::domain::Pokedex;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = Pokedex {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));

    cfg.service(
        web::resource("/api/pokedex/{version}/pokedex/{user_id}")
            .route(web::get().to(handle_paginated_pokemons::<Pokedex<PgRepository, PgService>>))
            .wrap(Auth),
    );

}
