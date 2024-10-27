use async_trait::async_trait;
use error::Error;
use support::store::models::pokemons::{pokemon::Pokemon, pokemon_ability::PokemonAbility, pokemon_stat::PokemonStat, pokemon_type::PokemonType};

use super::data::{pokemon_ability_data::PokemonAbilityUpdateData, pokemon_data::{PokemonAttributesDisplayData, PokemonShowData, PokemonUpdateData}, pokemon_pagination_data::PokemonAttributes, pokemon_stat_data::PokemonStatUpdateData, pokemon_type_data::PokemonTypeUpdateData};

#[async_trait]
pub trait AdminPokemonsContract{
    async fn paginated(&self, pokemon_attributes: PokemonAttributes) -> Result<Vec<Pokemon>, Error>;
    async fn show(&self, pokemon_id: &str) -> Result<PokemonShowData, Error>;
    async fn update(&self, pokemon_id: &str, pokemon_update_data: PokemonUpdateData) -> Result<Pokemon, Error>;
    async fn update_pokemon_ability(&self, pokemon_ability_id: &str, pokemon_ability_update_data: PokemonAbilityUpdateData) -> Result<PokemonAbility, Error>;
    async fn update_pokemon_stat(&self, pokemon_stat_id: &str, pokemon_stat_update_data: PokemonStatUpdateData) -> Result<PokemonStat, Error>;
    async fn update_pokemon_type(&self, pokemon_type_id: &str, pokemon_type_update_data: PokemonTypeUpdateData) -> Result<PokemonType, Error>;
}

// getters
#[async_trait]
pub trait PgRepositoryContract {
    async fn paginated(&self, pokemon_attributes: &PokemonAttributes) -> Result<Vec<Pokemon>, Error>;
    async fn get_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error>;
    async fn get_attributes(&self, pokemon_id:&str) -> Result<PokemonAttributesDisplayData, Error>;
    async fn get_existing_pokemon_update_ability_by_id(&self, pokemon_ability_id: &str) -> Result<PokemonAbilityUpdateData, Error>;
    async fn get_existing_pokemon_update_stat_by_id(&self, pokemon_stat_id: &str) -> Result<PokemonStatUpdateData, Error>;
    async fn get_existing_pokemon_update_type_by_id(&self, pokemon_type_id: &str) -> Result<PokemonTypeUpdateData, Error>;
}

// setters
#[async_trait]
pub trait PgServiceContract{
    async fn update(&self, pokemon_id: &str, pokemon_data: PokemonUpdateData) -> Result<Pokemon, Error>;
    async fn update_pokemon_ability(&self, pokemon_ability_id: &str, pokemon_ability_update_data: PokemonAbilityUpdateData) -> Result<PokemonAbility, Error>;
    async fn update_pokemon_stat(&self,  pokemon_stat_id: &str, pokemon_stat_update_data: PokemonStatUpdateData) -> Result<PokemonStat, Error>;
    async fn update_pokemon_type(&self,  pokemon_type_id: &str, pokemon_type_update_data: PokemonTypeUpdateData) -> Result<PokemonType, Error>;
}
