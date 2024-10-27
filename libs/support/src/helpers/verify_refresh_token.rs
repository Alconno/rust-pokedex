use crate::store::models::user::User;
use chrono::{NaiveDateTime, Utc};
use error::Error;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

/// Verify the refresh token.
pub fn verify_refresh_token(user: &User, refresh_token: &str) -> Result<(), Error> {
    // Retrieve the refresh token secret from configuration
    let refresh_token_secret = &config::get_default("REFRESH_TOKEN_SECRET", "")[..];

    // Create an HMAC key from the refresh token secret
    let key: Hmac<Sha256> = Hmac::new_from_slice(refresh_token_secret.to_string().as_bytes())
        .map_err(|_| Error::InternalError("Failed to create HMAC key".to_string()))?;

    // Verify the refresh token with the HMAC key
    let claims: BTreeMap<String, String> =
        refresh_token.verify_with_key(&key).map_err(|_| Error::Unauthorized("Unauthorized".to_string()))?;

    // Extract the expiration time and subject from the token claims
    let token_expires = claims
        .get("exp")
        .ok_or_else(|| Error::InternalError("Token expiration not found".to_string()))?;
    let token_sub = claims
        .get("sub")
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

    // Check if the user ID in the token matches the ID of the provided user
    if &user.id != token_sub {
        return Err(Error::Unauthorized("Unauthorized".to_string()));
    }

    Ok(())
}
