use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use validr::Validation;
use crate::application::admin::pokemons::{contract::AdminPokemonsContract, data::pokemon_type_data::RequestPokemonTypeUpdateData};

pub async fn handle_pokemon_type_update<T: AdminPokemonsContract>(
    req: HttpRequest,
    data: web::Json<RequestPokemonTypeUpdateData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract type_id from url
    let type_id = extract_uuid_after_keyword(&req, "types").unwrap_or("".to_string());

    // Get pokemon type update data
    let pokemon_type_update_data = data.into_inner().validate()?.insertable();

    // Get updated pokemon type
    let updated_pokemon_type = service.update_pokemon_type(&type_id, pokemon_type_update_data).await?;

    Ok(HttpResponse::Ok().json(updated_pokemon_type))
}