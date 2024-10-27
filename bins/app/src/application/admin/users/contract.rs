use async_trait::async_trait;
use error::Error;
use support::store::models::user::User;

use super::data::UserAttributes;



#[async_trait]
pub trait AdminUsersContract{
    async fn logout(&self, user_id: &str) -> Result<User, Error>;
    async fn deactivate(&self, user_id: &str) -> Result<User, Error>;
    async fn activate(&self, user_id: &str) -> Result<User, Error>;
    async fn paginated(&self, user_attributes: UserAttributes) -> Result<Vec<User>, Error>;
    async fn show(&self, user_id: &str) -> Result<User,Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn paginated(&self, user_attributes: UserAttributes) -> Result<Vec<User>, Error>;
    async fn get_by_id(&self, user_id: &str) -> Result<User, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract{
    async fn logout(&self, user_id: &str) -> Result<User, Error>;
    async fn deactivate(&self, user_id: &str) -> Result<User, Error>;
    async fn activate(&self, user_id: &str) -> Result<User, Error>;
}
