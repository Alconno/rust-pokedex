use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use validr::Validation;
use crate::application::admin::pokemons::{contract::AdminPokemonsContract, data::pokemon_data::RequestPokemonUpdateData};

pub async fn handle_pokemon_update<T: AdminPokemonsContract>(
    req: HttpRequest,
    data: web::Json<RequestPokemonUpdateData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pokemon_id from url
    let pokemon_id = extract_uuid_after_keyword(&req, "pokemons").unwrap_or("".to_string());

    // Get pokemon update data
    let pokemon_update_data = data.into_inner().validate()?.insertable();

    // Get updated pokemon
    let updated_pokemon = service.update(&pokemon_id, pokemon_update_data).await?;

    Ok(HttpResponse::Ok().json(updated_pokemon))
}