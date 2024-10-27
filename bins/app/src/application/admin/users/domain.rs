use super::{contract::{AdminUsersContract, PgRepositoryContract, PgServiceContract}, data::UserAttributes};
use async_trait::async_trait;
use error::Error;
use support::store::models::user::User;


/// Struct representing admin users.
pub struct AdminUsers<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> AdminUsersContract for AdminUsers<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Paginate users based on provided filters.
    async fn paginated(&self, user_attributes: UserAttributes) -> Result<Vec<User>, Error> {
        // Get paginated users from repository
        let users = self.repository.paginated(user_attributes).await?;

        Ok(users)
    }

    /// Show details of a specific user by user ID.
    async fn show(&self, user_id: &str) -> Result<User, Error> {
        Ok(self.repository.get_by_id(user_id).await?)
    }

    /// Logout a user by removing their refresh token.
    async fn logout(&self, user_id: &str) -> Result<User, Error> {
        Ok(self.service.logout(user_id).await?)
    }

    /// Delete a user by setting their deleted_at date.
    async fn deactivate(&self, user_id: &str) -> Result<User, Error> {
        Ok(self.service.deactivate(user_id).await?)
    }

    /// Activate a user by removing their deleted_at date.
    async fn activate(&self, user_id: &str) -> Result<User, Error> {
        Ok(self.service.activate(user_id).await?)
    }
}
