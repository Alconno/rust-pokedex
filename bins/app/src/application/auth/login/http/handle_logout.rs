use crate::application::auth::login::contract::LoginContract;
use actix_web::{cookie::{time, Cookie}, web, HttpRequest, HttpResponse};
use support::helpers::http::extract_token_by_name;
use error::Error;

pub async fn handle_logout<T: LoginContract>(
    service: web::Data<T>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // Extract the refresh token from the request
    let refresh_token = extract_token_by_name(&req, "refresh_token");

    // Call the logout method to invalidate the refresh token
    let logout_result = service.logout(&refresh_token).await;

    // Construct a cookie with an expired expiration time to remove it
    let remove_cookie = Cookie::build("tokens", "")
        .path("/")
        .max_age(time::Duration::minutes(0)).finish();

    match logout_result {
        Ok(_) => {
            Ok(HttpResponse::Ok()
                .cookie(remove_cookie)
                .json("User successfully logged out."))
        }
        Err(err) => {
            match err {
                Error::NotFoundWithCause(cause) if cause == "User with provided refresh token not found in database" => {
                    Ok(HttpResponse::Ok()
                        .cookie(remove_cookie)
                        .json("User successfully logged out. (User with provided refresh token was not found in database)"))
                }
                _ => {
                    Err(err)
                }
            }
        }
    }
}