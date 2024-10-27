use error::Error;
use async_trait::async_trait;
use support::store::models::{action_token::ActionToken, user::User};
use super::data::{UserData, UserValidationData};


#[async_trait]
pub trait RegisterContract{
    async fn register(&self, data: UserData, base_url: &str) -> Result<ActionToken, Error>;
    async fn resend_action_token(&self, data: UserValidationData, base_url: &str) -> Result<ActionToken, Error>;
    async fn check_action_token(&self, action_token_id: &str) -> Result<ActionToken, Error>;
    async fn verify_email(&self, action_token: ActionToken) -> Result<(), Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error>;
    async fn get_user_by_token_entity_id(&self, action_token_entity_id: &str) -> Result<User, Error>;
    async fn get_action_token_by_token(&self, action_token_id: &str) -> Result<ActionToken, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract {
    async fn register(&self, data: &UserData) -> Result<User, Error>;
    async fn create_action_token(&self, user_id: &str) -> Result<ActionToken, Error>;
    async fn user_update(&self, user: &User) -> Result<(), Error>;
    async fn action_token_update_executed_at(&self, action_token_id: &str) -> Result<(), Error>;
}