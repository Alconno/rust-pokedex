use crate::middleware::{Admin, Auth};

use super::domain::AdminPokemons;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = AdminPokemons {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));


    // Route to display paginated and filtered pokemons
    cfg.service(
        web::resource("/admin/pokemons")
            .route(web::get().to(handle_pokemons_paginated::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

     // Route to show specified pokemon
     cfg.service(
        web::resource("/admin/pokemons/{pokemon_id}")
            .route(web::get().to(handle_pokemon_show::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to update specified pokemon
    cfg.service(
        web::resource("/admin/pokemons/{pokemon_id}/update")
            .route(web::put().to(handle_pokemon_update::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to update specified pokemon_ability
    cfg.service(
        web::resource("/admin/pokemons/{pokemon_id}/abilities/{pokemon_ability_id}/update")
            .route(web::put().to(handle_pokemon_ability_update::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to update specified pokemon_stat
    cfg.service(
        web::resource("/admin/pokemons/{pokemon_id}/stats/{pokemon_stat_id}/update")
            .route(web::put().to(handle_pokemon_stat_update::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    // Route to update specified pokemon_type
    cfg.service(
        web::resource("/admin/pokemons/{pokemon_id}/types/{pokemon_stat_id}/update")
            .route(web::put().to(handle_pokemon_type_update::<AdminPokemons<PgRepository, PgService>>))
            .wrap(Admin)
            .wrap(Auth),
    );

    
}
