use super::{contract::{PgRepositoryContract, PgServiceContract, PokedexContract}, data::PokemonAttributes};
use async_trait::async_trait;
use error::Error;
use support::store::models::pokemons::pokemon::Pokemon;

pub struct Pokedex<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}


#[async_trait]
impl<A, B> PokedexContract for Pokedex<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Get paginated user pokemons.
    async fn paginated_user_pokemons(
        &self,
        user_id: &str,
        pokemon_attributes: PokemonAttributes,
    ) -> Result<Vec<Pokemon>, Error> {
        // Get paginated pokemons for current auth user
        let user_pokemons = self
            .repository
            .get_paginated_user_pokemons(user_id, pokemon_attributes)
            .await?;

        Ok(user_pokemons)
    }
}
