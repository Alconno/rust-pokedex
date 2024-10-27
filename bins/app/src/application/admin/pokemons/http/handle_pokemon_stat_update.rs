use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use validr::Validation;
use crate::application::admin::pokemons::{contract::AdminPokemonsContract, data::pokemon_stat_data::RequestPokemonStatUpdateData};

pub async fn handle_pokemon_stat_update<T: AdminPokemonsContract>(
    req: HttpRequest,
    data: web::Json<RequestPokemonStatUpdateData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract stat_id from url
    let stat_id = extract_uuid_after_keyword(&req, "stats").unwrap_or("".to_string());

    // Get pokemon stat update data
    let pokemon_stat_update_data = data.into_inner().validate()?.insertable();

    // Get updated pokemon stat
    let updated_pokemon_stat = service.update_pokemon_stat(&stat_id, pokemon_stat_update_data).await?;

    Ok(HttpResponse::Ok().json(updated_pokemon_stat))
}