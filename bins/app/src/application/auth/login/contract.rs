use async_trait::async_trait;
use error::Error;
use support::{store::models::jwt_tokens::JwtTokens, store::models::user::User};

use super::data::UserLoginValidationData;



#[async_trait]
pub trait LoginContract{
    async fn login(&self, data: UserLoginValidationData) -> Result<JwtTokens, Error>;
    async fn refresh(&self, refresh_token: &str) -> Result<String, Error>;
    async fn logout(&self, refresh_token: &str) -> Result<(), Error>;
    async fn check_user_login(&self, user_id: &str) -> bool;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
    async fn get_user_by_refresh_token(&self, token: &str) -> Result<User, Error>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract{
    async fn user_update(&self, user: &User) -> Result<(), Error>;
}
