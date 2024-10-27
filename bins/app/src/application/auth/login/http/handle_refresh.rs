use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse};
use error::Error;
use serde_json::json;
use support::helpers::http::extract_token_by_name;

use crate::application::auth::login::contract::LoginContract;

pub async fn handle_refresh<T: LoginContract>(
    service: web::Data<T>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Extract the refresh token from the request
    let refresh_token = extract_token_by_name(&request, "refresh_token");

    // Call the refresh method to generate a new access token
    let new_access_token = service.refresh(&refresh_token).await?;

    // Construct a new tokens JSON containing the new access token and the original refresh token
    let tokens_json = json!({
        "access_token": new_access_token,
        "refresh_token": refresh_token,
    });

    // Create a new tokens cookie containing the updated tokens JSON
    let tokens_cookie = Cookie::build("tokens", tokens_json.to_string())
        .path("/")
        .finish();
    
    // Return HTTP response with the new access token and updated tokens cookie
    Ok(HttpResponse::Ok()
        .cookie(tokens_cookie)
        .json(new_access_token))
}
