use async_trait::async_trait;
use error::Error;
use support::store::models::{pokemons::pokemon::Pokemon, user_attempts::UserAttempt};

use super::data::ResponseData;


#[async_trait]
pub trait PokemonGameContract{
    async fn get_random_pokemon(&self, user_id: &str) -> Result<ResponseData, Error>;
    async fn check_guess(&self, guess: &str, user_id: &str, attempt_id: &str) -> Result<(), Error>;
}


// getters
#[async_trait]
pub trait PgRepositoryContract{
    async fn get_blacklist(&self, user_id: &str) -> Result<Vec<i32>, Error>;
    async fn get_pokemon_by_pokemon_id(&self, pokemon_id: i32) -> Result<Pokemon, Error>;
    async fn get_attempt_by_user_id_and_pokemon_id(&self, user: &str, pokemon_id: &str) -> Result<UserAttempt, Error>;
    async fn check_attempt(&self, attempt_id: &str) -> Result<UserAttempt, Error>;
    async fn get_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error>;
}


// setters
#[async_trait]
pub trait PgServiceContract{
    async fn create_new_attempt(&self, pokemon_id: &str, user_id: &str) -> Result<UserAttempt, Error>;
    async fn create_pokemon_in_pokedex(&self, user_id: &str, pokemon_id: &str) -> Result<(), Error>;
    async fn update_attempt_to_successful(&self, attempt_id: &str) -> Result<(), Error>;

}