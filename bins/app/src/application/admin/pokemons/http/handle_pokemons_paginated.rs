use actix_web::{web, HttpResponse};
use error::Error;
use validr::Validation;
use crate::application::admin::pokemons::{contract::AdminPokemonsContract, data::pokemon_pagination_data::RequestPokemonAttributes};


pub async fn handle_pokemons_paginated<T: AdminPokemonsContract>(
    query_params: web::Query<RequestPokemonAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pagination filters from request data
    let paginaton_filters = query_params.into_inner().validate()?;

    // Get paginated pokemons
    let pokemons = service.paginated(paginaton_filters.into()).await?;

    Ok(HttpResponse::Ok().json(pokemons))
}