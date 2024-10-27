use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use crate::application::admin::pokemons::contract::AdminPokemonsContract;

pub async fn handle_pokemon_show<T: AdminPokemonsContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pokemon_id from request
    let pokemon_id = extract_uuid_after_keyword(&req, "pokemons").unwrap_or("".to_string());

    // Get specific pokemon by id
    let pokemon = service.show(&pokemon_id).await?;

    Ok(HttpResponse::Ok().json(pokemon))
}