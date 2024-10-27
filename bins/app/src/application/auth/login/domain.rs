use super::{contract::{LoginContract, PgRepositoryContract, PgServiceContract}, data::UserLoginValidationData};
use async_trait::async_trait;
use error::Error;
use pwhash::bcrypt;
use support::{
    helpers::{generate_tokens, verify_refresh_token::verify_refresh_token},
    store::models::jwt_tokens::JwtTokens,
};

/// Struct representing the login functionality.
pub struct Login<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> LoginContract for Login<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Method to handle user login.
    async fn login(&self, data: UserLoginValidationData) -> Result<JwtTokens, Error> {
        match self.repository.get_user_by_email(&data.email).await {
            Ok(mut user) => {
                // Verify user password
                if !bcrypt::verify(data.password, &user.password) {
                    return Err(Error::BadRequest("Bad request".to_string()));
                }

                // Check if email is verified
                if user.email_verified_at.is_none() {
                    return Err(Error::BadRequest("Email not verified".to_string()));
                }

                // Generate tokens for the user
                let tokens = generate_tokens::generate_tokens(&user.id, &user.role)?;
                
                // Update user refresh token
                user.refresh_token = Some(tokens.refresh_token.clone());
                self.service.user_update(&user).await?;

                Ok(tokens)
            }
            Err(_) => Err(Error::NotFoundWithCause(
                "User with provided email not found in database".to_string()
            ))
        }
    }

    /// Method to handle token refresh.
    async fn refresh(&self, refresh_token: &str) -> Result<String, Error> {
        match self.repository.get_user_by_refresh_token(refresh_token).await {
            Ok(user) => {
                // Verify refresh token
                verify_refresh_token(&user, refresh_token)?;
                // Generate new access token
                let tokens = generate_tokens::generate_tokens(&user.id, &user.role)?;
                Ok(tokens.access_token)
            }
            Err(_) => {
                if refresh_token.is_empty() {
                    return Err(Error::NotFoundWithCause("Refresh token not found in cookies".to_string()))
                }
                Err(Error::NotFoundWithCause(
                    "User with provided refresh token not found in database".to_string()
                ))
            }
        }
    }

    /// Method to handle user logout.
    async fn logout(&self, refresh_token: &str) -> Result<(), Error> {
        match self.repository.get_user_by_refresh_token(refresh_token).await {
            Ok(mut user) => {
                // Remove user refresh token
                user.refresh_token = None;
                self.service.user_update(&user).await?;
                Ok(())
            }
            Err(_) => {
                if refresh_token.is_empty() {
                    return Err(Error::NotFoundWithCause("Refresh token not found in cookies".to_string()))
                }
                Err(Error::NotFoundWithCause(
                    "User with provided refresh token not found in database".to_string()
                ))
            }
        }
    }

    /// Method to check if user is already logged in
    async fn check_user_login(&self, user_id: &str) -> bool{
        match self.repository.get_user_by_id(user_id).await{
            Ok(_) => true,
            _ => false,
        }
    }
}
