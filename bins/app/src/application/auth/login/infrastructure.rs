use std::sync::Arc;

use super::contract::{PgRepositoryContract, PgServiceContract};
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Pg;
use support::store::models::user::User;



pub struct PgRepository{
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository{

    /// Retrieves a user from the database by their email address.
    async fn get_user_by_email(&self, email:&str) -> Result<User, Error>{
        User::get_by_email(email, &mut self.pg_pool.connection()?)
    }

    // Retrieves a user from the database by their refresh token.
    async fn get_user_by_refresh_token(&self, token:&str) -> Result<User, Error>{
        User::get_by_refresh_token(token, &mut self.pg_pool.connection()?)
    }

    // Retrieves a user from database by their id
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, Error>{
        User::get_by_id(user_id, &mut self.pg_pool.connection()?)
    }
}


pub struct PgService{
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService{

    // Update the user in the database using the provided user object
    async fn user_update(&self, user: &User) -> Result<(), Error> {
        User::update_user(user, &mut self.pg_pool.connection()?)?;
        Ok(())
    }

}