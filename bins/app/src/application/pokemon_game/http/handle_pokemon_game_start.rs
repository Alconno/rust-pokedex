use actix_web::{HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{get_api_version, get_authenticated_user_id_from_request};
use actix_web::web;
use crate::application::pokemon_game::contract::PokemonGameContract;

pub async fn handle_pokemon_game_start<T: PokemonGameContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    match get_api_version(&req).as_ref() {
        "v1" => handle_pokemon_game_start_v1(req, service).await,
        _ => Err(Error::NotFound),
    }
}

pub async fn handle_pokemon_game_start_v1<T: PokemonGameContract>(
    req: HttpRequest,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let authenticated_user_id = get_authenticated_user_id_from_request(&req)?;

    // Call the get_random_pokemon method on the mutable reference to the service
    let response = service.get_random_pokemon(&authenticated_user_id).await?;

    Ok(HttpResponse::Ok().json(response))
}