use super::{
    contract::{AdminPokemonsContract, PgRepositoryContract, PgServiceContract}, data::{pokemon_ability_data::PokemonAbilityUpdateData, pokemon_data::{PokemonShowData, PokemonUpdateData}, 
    pokemon_pagination_data::PokemonAttributes, pokemon_stat_data::PokemonStatUpdateData, pokemon_type_data::PokemonTypeUpdateData},
};
use async_trait::async_trait;
use error::Error;
use support::store::models::pokemons::{pokemon::Pokemon, pokemon_ability::PokemonAbility, pokemon_stat::PokemonStat, pokemon_type::PokemonType};




/// Structure for managing admin pokemons.
pub struct AdminPokemons<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl<A, B> AdminPokemonsContract for AdminPokemons<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Paginate pokemons.
    async fn paginated(&self, pokemon_attributes: PokemonAttributes) -> Result<Vec<Pokemon>, Error> {
        // Get paginated pokemons from repository
        let pokemons = self.repository.paginated(&pokemon_attributes).await?;

        Ok(pokemons)
    }

    /// Get detailed information about a specific pokemon.
    async fn show(&self, pokemon_id: &str) -> Result<PokemonShowData, Error> {
        let pokemon = self.repository.get_by_id(pokemon_id).await?;
        let attributes = self.repository.get_attributes(pokemon_id).await?;

        Ok(PokemonShowData { pokemon, attributes })
    }

    /// Update a pokemon.
    async fn update(&self, pokemon_id: &str, pokemon_update_data: PokemonUpdateData) -> Result<Pokemon, Error> {
        Ok(self.service.update(pokemon_id, pokemon_update_data).await?)
    }

    /// Update a pokemon's ability.
    async fn update_pokemon_ability(&self, pokemon_ability_id: &str, mut pokemon_ability_update_data: PokemonAbilityUpdateData) -> Result<PokemonAbility, Error> {
        let existing_ability = self.repository.get_existing_pokemon_update_ability_by_id(pokemon_ability_id).await?;

        pokemon_ability_update_data.merge(&existing_ability);

        Ok(self.service.update_pokemon_ability(pokemon_ability_id, pokemon_ability_update_data).await?)
    }

    /// Update a pokemon's stat.
    async fn update_pokemon_stat(&self, pokemon_stat_id: &str, mut pokemon_stat_update_data: PokemonStatUpdateData) -> Result<PokemonStat, Error> {
        let existing_stat = self.repository.get_existing_pokemon_update_stat_by_id(pokemon_stat_id).await?;

        pokemon_stat_update_data.merge(&existing_stat);

        Ok(self.service.update_pokemon_stat(pokemon_stat_id, pokemon_stat_update_data).await?)
    }

    /// Update a pokemon's type.
    async fn update_pokemon_type(&self, pokemon_type_id: &str, mut pokemon_type_update_data: PokemonTypeUpdateData) -> Result<PokemonType, Error> {
        let existing_type = self.repository.get_existing_pokemon_update_type_by_id(pokemon_type_id).await?;

        pokemon_type_update_data.merge(&existing_type);

        Ok(self.service.update_pokemon_type(pokemon_type_id, pokemon_type_update_data).await?)
    }
}
