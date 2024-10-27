use actix_web::{HttpMessage, HttpRequest};
use error::Error;
use std::str::FromStr;


/// Parse given parameter easily from the request path string
pub fn part_from_path<T: FromStr>(req: &HttpRequest, name: &str) -> Result<T, Error> {
    let value: T = match req.match_info().get(name) {
        Some(val) => match val.parse() {
            Ok(v) => v,
            Err(_) => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
        },
        None => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
    };

    Ok(value)
}

pub fn extract_token_by_name<T>(req: &T, token_name: &str) -> String
where
    T: HttpMessage,
{
    // Get the cookie header from the request
    let cookie_header = match req.headers().get("cookie") {
        Some(header) => header,
        None => return "".to_string(), // No cookie header found, return an empty string
    };

    // Convert the cookie header to a string
    let cookie_str = match cookie_header.to_str() {
        Ok(s) => s, 
        Err(_) => return "".to_string(), // Failed to convert to string, return an empty string
    };

    // Find the "tokens" cookie in the cookie string
    if let Some(tokens_cookie) = cookie_str.split(';').find(|c| c.trim().starts_with("tokens=")) {
        // Parse the tokens JSON from the cookie
        if let Some(tokens_json) = tokens_cookie.split('=').nth(1) {
            // Parse the JSON string into a serde_json::Value
            if let Ok(tokens_value) = serde_json::from_str::<serde_json::Value>(tokens_json) {
                // Extract the "access_token" field from the tokens
                if let Some(access_token) = tokens_value.get(token_name) {
                    if let Some(access_token_str) = access_token.as_str() {
                        return access_token_str.to_string() // Return the access token string
                    }
                }
            }
        }
    }

    // No access token found, return an empty string
    "".to_string()
}


// Retrieve authenticated user id from request
pub fn get_authenticated_user_id_from_request(req: &HttpRequest) -> Result<String, Error> {
    match req.extensions().get::<String>() {
        Some(user_id) => Ok(user_id.clone()),
        None => Err(Error::Unauthorized("Unauthorized".to_string())),
    }
}

// Retrieve user id from request
pub fn get_user_id_from_request(req: &HttpRequest) -> Result<String, Error> {
    match req.extensions().get::<String>() {
        Some(user_id) => Ok(user_id.clone()),
        None => Err(Error::NotFoundWithCause("User Id Not Found in Request".to_string())),
    }
}



/// Get api version from the route path
pub fn get_api_version(req: &HttpRequest) -> String {
    match req.match_info().get("version") {
        Some(value) => value.parse().unwrap_or_else(|_| "v1".to_string()),
        None => "v1".to_string(),
    }
}