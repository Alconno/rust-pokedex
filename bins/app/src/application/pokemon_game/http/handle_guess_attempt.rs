use actix_web::{HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{get_api_version, get_authenticated_user_id_from_request, part_from_path};
use actix_web::web;
use validr::Validation;
use crate::application::pokemon_game::{contract::PokemonGameContract, data::RequestAttemptData};


pub async fn handle_guess_attempt<T: PokemonGameContract>(
    req: HttpRequest,
    data: web::Json<RequestAttemptData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    match get_api_version(&req).as_ref() {
        "v1" => handle_guess_attempt_v1(req, data, service).await,
        _ => Err(Error::NotFound),
    }
}

pub async fn handle_guess_attempt_v1<T: PokemonGameContract>(
    req: HttpRequest,
    data: web::Json<RequestAttemptData>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract guess from request data
    let guess = data.into_inner().validate()?.guess.unwrap_or("".to_string());

    // Get authenticated user ID from request
    let authenticated_user_id = get_authenticated_user_id_from_request(&req)?;

    // Extract attempt ID from request path
    let attempt_id = part_from_path::<String>(&req, "attempt_id")?;

    // Check guess against pokedex
    service
        .check_guess(&guess, &authenticated_user_id, &attempt_id)
        .await?;

    Ok(HttpResponse::Ok().json("Pokemon saved in pokedex."))
}
