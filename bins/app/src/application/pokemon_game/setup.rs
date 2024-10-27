use crate::middleware::Auth;

use super::domain::PokemonGame;
use super::http::*;
use super::infrastructure::{PgRepository, PgService};
use actix_web::web;
use infrastructure::db::Pg;
use std::sync::Arc;

pub fn routes(pg_pool: Arc<Pg>, cfg: &mut web::ServiceConfig) {
    let service = PokemonGame {
        service: PgService {
            pg_pool: pg_pool.clone(),
        },
        repository: PgRepository { pg_pool },
    };
    
    cfg.app_data(web::Data::new(service));

    cfg.service(
        web::resource("/api/pokedex/{version}/pokemon")
            .route(web::get().to(handle_pokemon_game_start::<PokemonGame<PgRepository, PgService>>))
            .wrap(Auth),
    );
    cfg.service(
        web::resource("/api/pokedex/{version}/pokemon/attempt/{attempt_id}")
            .route(web::post().to(handle_guess_attempt::<PokemonGame<PgRepository, PgService>>))
            .wrap(Auth),
    );
}
