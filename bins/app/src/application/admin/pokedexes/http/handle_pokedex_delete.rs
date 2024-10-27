use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use crate::application::admin::pokedexes::contract::AdminPokedexesContract;

pub async fn handle_pokedex_delete<T: AdminPokedexesContract>(
    service: web::Data<T>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {

    // Extract data from request
    let user_pokedex_id = extract_uuid_after_keyword(&req, "pokedexes").unwrap_or("".to_string());

    // Delete user pokedex and set attempt success to false
    service.delete(&user_pokedex_id).await?;

    // Return paginated pokedex as JSON response
    Ok(HttpResponse::Ok().json("User pokedex successfully deleted.".to_string()))
}