use actix_web::{HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::{get_api_version, get_authenticated_user_id_from_request};
use actix_web::web;
use validr::Validation;
use crate::application::pokedex::{contract::PokedexContract, data::RequestPokemonAttributes};


pub async fn handle_paginated_pokemons<T: PokedexContract>(
    req: HttpRequest,
    query_params: web::Query<RequestPokemonAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    match get_api_version(&req).as_ref() {
        "v1" => handle_paginated_pokemons_v1(req, query_params, service).await,
        _ => Err(Error::NotFound),
    }
}
pub async fn handle_paginated_pokemons_v1<T: PokedexContract>(
    req: HttpRequest,
    query_params: web::Query<RequestPokemonAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pagination filters from request data
    let paginaton_filters = query_params.into_inner().validate()?;

    // Get authenticated user ID from request
    let authenticated_user_id = get_authenticated_user_id_from_request(&req)?;

    // Get paginated pokemons for the authenticated user
    let pokemons = service.paginated_user_pokemons(&authenticated_user_id, paginaton_filters.into()).await?;

    // Return paginated pokemons as JSON response
    Ok(HttpResponse::Ok().json(pokemons))
}