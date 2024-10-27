use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use validr::Validation;
use crate::application::admin::pokemons::{contract::AdminPokemonsContract, data::pokemon_ability_data::RequestPokemonAbilityUpdateData};

pub async fn handle_pokemon_ability_update<T: AdminPokemonsContract>(
    req: HttpRequest,
    data: web::Json<RequestPokemonAbilityUpdateData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract ability_id from url
    let ability_id = extract_uuid_after_keyword(&req, "abilities").unwrap_or("".to_string());

    // Get pokemon ability update data
    let pokemon_ability_update_data = data.into_inner().validate()?.insertable();

    // Get updated pokemon ability 
    let updated_pokemon_ability = service.update_pokemon_ability(&ability_id, pokemon_ability_update_data).await?;

    Ok(HttpResponse::Ok().json(updated_pokemon_ability))
}