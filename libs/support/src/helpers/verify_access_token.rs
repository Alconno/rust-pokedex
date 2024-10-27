use chrono::{NaiveDateTime, Utc};
use error::Error;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

/// Verify the access token.
pub fn verify_access_token(access_token: &str) -> Result<(String, String), Error> {
    // Retrieve the access token secret from configuration
    let access_token_secret = &config::get_default("ACCESS_TOKEN_SECRET", "")[..];
    // Create an HMAC key from the access token secret
    let key: Hmac<Sha256> = Hmac::new_from_slice(access_token_secret.to_string().as_bytes())
        .map_err(|_| Error::InternalError("Failed to create HMAC key".to_string()))?;

    // Verify the access token with the HMAC key
    let claims: BTreeMap<String, String> =
        access_token.verify_with_key(&key).map_err(|_| Error::Unauthorized("Unauthorized".to_string()))?;

    // Extract the expiration time and subject from the token claims
    let token_expires = claims
        .get("exp")
        .ok_or_else(|| Error::InternalError("Token expiration not found".to_string()))?;
    let token_sub = claims
        .get("sub")
        .ok_or_else(|| Error::InternalError("Token subject not found".to_string()))?;
    let token_role = claims
        .get("role")
        .ok_or_else(|| Error::InternalError("Token subject not found".to_string()))?;

    // Parse the expiration time into a NaiveDateTime
    let token_expires = match NaiveDateTime::parse_from_str(token_expires, "%Y-%m-%d %H:%M:%S%.f %Z") {
        Ok(exp) => exp,
        Err(_) => {
            return Err(Error::InternalError("Access Token parsing error".to_string()));
        }
    };

    // Check if the token has expired
    if token_expires <= Utc::now().naive_utc() {
        return Err(Error::Unauthorized("Access token expired".to_string()));
    }

    Ok((token_sub.clone(), token_role.clone()))
}


/// Extracts the user ID (sub key) from the access token.
pub fn extract_user_id_from_access_token(access_token: &str) -> Result<String, Error> {
    // Retrieve the access token secret from configuration
    let access_token_secret = &config::get_default("ACCESS_TOKEN_SECRET", "")[..];
    // Create an HMAC key from the access token secret
    let key: Hmac<Sha256> = Hmac::new_from_slice(access_token_secret.to_string().as_bytes())
        .map_err(|_| Error::InternalError("Failed to create HMAC key".to_string()))?;

    // Verify the access token with the HMAC key
    let claims: BTreeMap<String, String> =
        access_token.verify_with_key(&key).map_err(|_| Error::Unauthorized("Unauthorized".to_string()))?;

    // Extract the user ID (sub) from the token claims
    let user_id = claims
        .get("sub")
        .ok_or_else(|| Error::InternalError("User ID (sub) not found in token claims".to_string()))?
        .clone();

    Ok(user_id)
}