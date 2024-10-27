use std::sync::Arc;

use super::{
    contract::{PgRepositoryContract, PgServiceContract},
    data::{ActionTokenData, UserData},
};
use async_trait::async_trait;
use diesel::RunQueryDsl;
use error::Error;
use infrastructure::{
    db::Pg,
    schema::{action_tokens, users},
};
use support::helpers::generate_tokens;
use support::store::models::{action_token::ActionToken, user::User};




pub struct PgRepository{
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository{
    async fn get_user_by_email(&self, email:&str) -> Result<User, Error>{
        User::get_by_email(email, &mut self.pg_pool.connection()?)
    }

    async fn get_user_by_token_entity_id(&self, action_token_entity_id: &str) -> Result<User, Error>{
        User::get_by_id(&action_token_entity_id, &mut self.pg_pool.connection()?)
    }
    
    async fn get_action_token_by_token(&self, action_token_id:&str) -> Result<ActionToken, Error>{
        ActionToken::get_by_token(action_token_id, &mut self.pg_pool.connection()?)
    }
}


pub struct PgService{
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService{
    // Create new user (registration)
    async fn register(&self, data: &UserData) -> Result<User, Error>{
        diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(&mut self.pg_pool.connection()?)
            .map_err(Error::from)
    }

    // Create new action token
    async fn create_action_token(&self, user_id: &str) -> Result<ActionToken, Error>{
        let data = ActionTokenData{
            entity_id: user_id.to_string(),
            action_name: "email verification".to_string(),
            token: generate_tokens::generate_random_action_token(10)
        };
        diesel::insert_into(action_tokens::table)
            .values(data)
            .get_result::<ActionToken>(&mut self.pg_pool.connection()?)
            .map_err(Error::from)
    }

    // Update action token
    async fn action_token_update_executed_at(&self, action_token_id: &str) -> Result<(), Error>{
        ActionToken::action_token_update_executed_at(action_token_id, &mut self.pg_pool.connection()?).await?;
        Ok(())
    }

    // update user
    async fn user_update(&self, user: &User) -> Result<(), Error> {
        User::update_user(user, &mut self.pg_pool.connection()?)?;
        Ok(())
    }

}