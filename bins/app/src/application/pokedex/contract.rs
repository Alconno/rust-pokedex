use async_trait::async_trait;
use error::Error;
use support::store::models::pokemons::pokemon::Pokemon;

use super::data::PokemonAttributes;

#[async_trait]
pub trait PokedexContract{
    async fn paginated_user_pokemons(&self, user_id: &str, pokemon_attributes: PokemonAttributes) -> Result<Vec<Pokemon>, Error>;
}


// getters
#[async_trait]
pub trait PgRepositoryContract{
    async fn get_paginated_user_pokemons(&self, user_id: &str, pokemon_attributes: PokemonAttributes) -> Result<Vec<Pokemon>, Error>;
}


// setters
#[async_trait]
pub trait PgServiceContract{

}