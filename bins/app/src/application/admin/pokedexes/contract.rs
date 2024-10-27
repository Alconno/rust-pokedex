use async_trait::async_trait;
use error::Error;
use support::store::models::{user_attempts::UserAttempt, user_pokedex::{UserPokedex, UserPokedexData}};

use super::data::PokedexAttributes;

#[async_trait]
pub trait AdminPokedexesContract{
    async fn paginated(&self, pokedex_attributes: PokedexAttributes) -> Result<Vec<UserPokedex>, Error>;
    async fn delete(&self, user_pokedex_id: &str) -> Result<(), Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn paginated(&self, pokedex_attributes: &PokedexAttributes) -> Result<Vec<UserPokedex>, Error>;
    async fn get_user_attempt_by_user_id_and_pokemon_id(&self, user_id: &str, pokemon_id: &str) -> Result<UserAttempt, Error>;
    async fn get_by_id(&self, user_pokedex_id: &str) -> Result<UserPokedexData, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract{
    async fn delete(&self, user_pokedex_id: &str) -> Result<(), Error>;
    async fn set_attempt_success_false(&self, user_attempt_id: &str) -> Result<(), Error>;
}
