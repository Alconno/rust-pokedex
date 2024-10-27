use super::{
    contract::{PgRepositoryContract, PgServiceContract, RegisterContract},
    data::{UserData, UserValidationData},
};
use async_trait::async_trait;
use chrono::Utc;
use error::Error;
use support::helpers::send_email;
use support::store::models::action_token::ActionToken;
use pwhash::bcrypt;

// Define a struct for user registration
pub struct Register<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> RegisterContract for Register<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    // Method for user registration
    async fn register(&self, data: UserData, base_url: &str) -> Result<ActionToken, Error> {
        match self.repository.get_user_by_email(&data.email).await {
            Ok(_) => Err(Error::BadRequest("User already exists".to_string())),

            // Register user if not already exists
            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => {
                let user = self.service.register(&data).await?;
                let action_token = self.service.create_action_token(&user.id).await?;
                send_email::send(&action_token.token, &data.email, base_url)?;

                Ok(action_token)
            }
            Err(e) => Err(e),
        }
    }

    // Method for resending action token for email verification
    async fn resend_action_token(
        &self,
        data: UserValidationData,
        base_url: &str,
    ) -> Result<ActionToken, Error> {
        let user = self.repository.get_user_by_email(&data.email).await?;
        if !bcrypt::verify(&data.password, &user.password) {
            return Err(Error::BadRequest("Incorrect password".to_string()));
        }

        if let Some(_) = user.email_verified_at {
            return Err(Error::BadRequest("Email already verified".to_string()));
        }

        let action_token = self.service.create_action_token(&user.id).await?;
        send_email::send(&action_token.token, &data.email, base_url)?;

        Ok(action_token)
    }

    // Method for checking the validity of an action token
    async fn check_action_token(&self, action_token_id: &str) -> Result<ActionToken, Error> {
        let action_token = self.repository.get_action_token_by_token(action_token_id).await?;

        if action_token.executed_at.is_some() {
            return Err(Error::BadRequest("Token already executed".to_string()));
        }

        if action_token.expires_at.timestamp() < Utc::now().timestamp() {
            return Err(Error::BadRequest("Token expired".to_string()));
        }
        Ok(action_token)
    }

    // Method for verifying email using action token
    async fn verify_email(&self, action_token: ActionToken) -> Result<(), Error> {
        match self.repository.get_user_by_token_entity_id(&action_token.entity_id).await {
            Ok(mut user) => {
                if user.email_verified_at.is_some() {
                    return Err(Error::BadRequest("Email already verified".to_string()));
                }

                user.email_verified_at = Some(Utc::now().naive_utc());
                self.service.user_update(&user).await?;

                self.service.action_token_update_executed_at(&action_token.id).await?;

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}