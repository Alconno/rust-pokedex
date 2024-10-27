use actix_web::{web, HttpResponse};
use error::Error;
use validr::Validation;
use crate::application::admin::users::{contract::AdminUsersContract, data::RequestUserAttributes};

pub async fn handle_users_paginated<T: AdminUsersContract>(
    query_params: web::Query<RequestUserAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    // Extract pagination filters from request data
    let paginaton_filters = query_params.into_inner().validate()?;

    // Get paginated users for the authenticated user
    let users = service.paginated(paginaton_filters.into()).await?;
    
    Ok(HttpResponse::Ok().json(users))
}