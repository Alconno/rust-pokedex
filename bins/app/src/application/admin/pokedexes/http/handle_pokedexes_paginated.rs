use actix_web::{web, HttpResponse};
use error::Error;
use validr::Validation;
use crate::application::admin::pokedexes::{contract::AdminPokedexesContract, data::RequestPokedexAttributes};

pub async fn handle_pokedexes_paginated<T: AdminPokedexesContract>(
    query_params: web::Query<RequestPokedexAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pagination filters from request data
    let paginaton_filters = query_params.into_inner().validate()?;

    // Get paginated pokedex
    let pokedexes = service.paginated(paginaton_filters.into()).await?;

    // Return paginated pokedex as JSON response
    Ok(HttpResponse::Ok().json(pokedexes))
}