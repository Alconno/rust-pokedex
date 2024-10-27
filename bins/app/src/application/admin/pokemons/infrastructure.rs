use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use error::Error;
use infrastructure::{
    db::Pg,
    schema::{pokemon_abilities, pokemon_stats, pokemon_types, pokemons},
};
use support::store::models::pokemons::{pokemon::Pokemon, pokemon_ability::PokemonAbility, pokemon_stat::PokemonStat, pokemon_type::PokemonType};
use super::{
    contract::{PgRepositoryContract, PgServiceContract}, data::{pokemon_ability_data::{PokemonAbilityDisplayData, PokemonAbilityUpdateData}, pokemon_data::{PokemonAttributesDisplayData, PokemonUpdateData}, pokemon_pagination_data::PokemonAttributes, pokemon_stat_data::{PokemonStatDisplayData, PokemonStatUpdateData}, pokemon_type_data::{PokemonTypeDisplayData, PokemonTypeUpdateData}},
};



pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Get paginated pokemons.
    async fn paginated(
        &self,
        pokemon_attributes: &PokemonAttributes,
    ) -> Result<Vec<Pokemon>, Error> {
        // Build the base query
        let mut query = pokemons::table.into_boxed();

        // Apply search to the query
        if !pokemon_attributes.search.is_empty(){
            query = query.filter(pokemons::name.like(format!("%{}%", &pokemon_attributes.search)));
        }

        // Apply sorting to the query
        query = match &*pokemon_attributes.sort_by {
            "created_at" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::created_at.asc()),
                _ => query.order(pokemons::created_at.desc()),
            },
            "updated_at" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::updated_at.asc()),
                _ => query.order(pokemons::updated_at.desc()),
            },
            "name" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::name.asc()),
                _ => query.order(pokemons::name.desc()),
            },
            "base_experience" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::base_experience.asc()),
                _ => query.order(pokemons::base_experience.desc()),
            },
            "height" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::height.asc()),
                _ => query.order(pokemons::height.desc()),
            },
            "weight" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::weight.asc()),
                _ => query.order(pokemons::weight.desc()),
            },
            _ => query,
        };

        // Apply pagination and return result
        match query
            .offset((pokemon_attributes.page - 1) * pokemon_attributes.per_page)
            .limit(pokemon_attributes.per_page)
            .load::<Pokemon>(&mut self.pg_pool.connection()?)
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::InternalError(format!(
                "Failed fetching pokemons for pagination. Error: {}",
                e.to_string()
            ))),
        }
    }

    /// Get a Pokemon by its ID.
    async fn get_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error> {
        Ok(Pokemon::get_by_id(pokemon_id, &mut self.pg_pool.connection()?).await?)
    }

    /// Get attributes of a Pokemon by its ID.
    async fn get_attributes(&self, pokemon_id: &str) -> Result<PokemonAttributesDisplayData, Error> {
        let mut connection = self.pg_pool.connection()?;

        let abilities = match PokemonAbility::get_all_by_pokemon_id(pokemon_id, &mut connection).await {
            Ok(abilities) => abilities
                .into_iter()
                .map(|ability| PokemonAbilityDisplayData {
                    id: ability.id,
                    name: ability.name,
                })
                .collect(),
            Err(e) => return Err(Error::InternalError(format!("Failed fetching all pokemon abilities by pokemon_id. Error: {}", e.to_string()))),
        };

        let stats = match PokemonStat::get_all_by_pokemon_id(pokemon_id, &mut connection).await {
            Ok(stats) => stats
                .into_iter()
                .map(|stat| PokemonStatDisplayData {
                    id: stat.id,
                    name: stat.name,
                    base_stat: stat.base_stat,
                    effort: stat.effort,
                })
                .collect(),
            Err(e) => return Err(Error::InternalError(format!("Failed fetching all pokemon stats by pokemon_id. Error: {}", e.to_string()))),
        };

        let types = match PokemonType::get_all_by_pokemon_id(pokemon_id, &mut connection).await {
            Ok(types) => types
                .into_iter()
                .map(|type_| PokemonTypeDisplayData {
                    id: type_.id,
                    name: type_.name,
                    slot: type_.slot,
                })
                .collect(),
            Err(e) => return Err(Error::InternalError(format!("Failed fetching all pokemon types by pokemon_id. Error: {}", e.to_string()))),
        };

        Ok(PokemonAttributesDisplayData { abilities, stats, types })
    }

    /// Get existing Pokemon ability update data by ID.
    async fn get_existing_pokemon_update_ability_by_id(&self, pokemon_ability_id: &str) -> Result<PokemonAbilityUpdateData, Error> {
        let mut connection = self.pg_pool.connection()?;
    
        let ability_update_data = match pokemon_abilities::table
            .select(pokemon_abilities::name)
            .filter(pokemon_abilities::id.eq(pokemon_ability_id))
            .get_result::<Option<String>>(&mut connection)
        {
            Ok(existing_pokemon_ability_update_data) => existing_pokemon_ability_update_data,
            Err(e) => {
                return Err(Error::NotFoundWithCause(format!(
                    "Failed fetching pokemon_ability update data by id. Error: {}",
                    e.to_string()
                )))
            }
        };
    
        Ok(PokemonAbilityUpdateData {
            name: ability_update_data.unwrap_or("".to_string()),
        })
    }

    /// Get existing Pokemon stat update data by ID.
    async fn get_existing_pokemon_update_stat_by_id(&self, pokemon_stat_id: &str) -> Result<PokemonStatUpdateData, Error> {
        let mut connection = self.pg_pool.connection()?;
    
        let update_data = match pokemon_stats::table
            .select((
                pokemon_stats::name,
                pokemon_stats::base_stat,
                pokemon_stats::effort,
            ))
            .filter(pokemon_stats::id.eq(pokemon_stat_id))
            .get_result::<(Option<String>, Option<i32>, Option<i32>)>(&mut connection)
        {
            Ok(existing_pokemon_stat_update_data) => existing_pokemon_stat_update_data,
            Err(e) => {
                return Err(Error::NotFoundWithCause(format!(
                    "Failed fetching pokemon_stat update data by id. Error: {}",
                    e.to_string()
                )))
            }
        };
    
        Ok(PokemonStatUpdateData {
            name: update_data.0.unwrap_or("".to_string()),
            base_stat: update_data.1.unwrap_or(0),
            effort: update_data.2.unwrap_or(0),
        })
    }

    /// Get existing Pokemon type update data by ID.
    async fn get_existing_pokemon_update_type_by_id(&self, pokemon_type_id: &str) -> Result<PokemonTypeUpdateData, Error> {
        let mut connection = self.pg_pool.connection()?;
    
        let update_data = match pokemon_types::table
            .select((pokemon_types::name, pokemon_types::slot))
            .filter(pokemon_types::id.eq(pokemon_type_id))
            .get_result::<(Option<String>, Option<i32>)>(&mut connection)
        {
            Ok(existing_pokemon_type_update_data) => existing_pokemon_type_update_data,
            Err(e) => {
                return Err(Error::NotFoundWithCause(format!(
                    "Failed fetching pokemon_type update data by id. Error: {}",
                    e.to_string()
                )))
            }
        };
    
        Ok(PokemonTypeUpdateData {
            name: update_data.0.unwrap_or("".to_string()),
            slot: update_data.1.unwrap_or(0),
        })
    }
}



pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    /// Update a Pokemon.
    async fn update(&self, pokemon_id: &str, pokemon_data: PokemonUpdateData) -> Result<Pokemon, Error> {
        let updated_pokemon = diesel::update(pokemons::dsl::pokemons
            .filter(pokemons::dsl::id.eq(&pokemon_id)))
            .set(pokemon_data)
            .get_result::<Pokemon>(&mut self.pg_pool.connection()?)
            .map_err(Error::from)?;
        Ok(updated_pokemon)
    }

    /// Update a Pokemon ability.
    async fn update_pokemon_ability(&self, pokemon_ability_id: &str, pokemon_ability_update_data: PokemonAbilityUpdateData) -> Result<PokemonAbility, Error> {
        let updated_ability = diesel::update(pokemon_abilities::dsl::pokemon_abilities
            .filter(pokemon_abilities::dsl::id.eq(&pokemon_ability_id)))
            .set(pokemon_ability_update_data)
            .get_result::<PokemonAbility>(&mut self.pg_pool.connection()?)?;
        Ok(updated_ability)
    }

    /// Update a Pokemon stat.
    async fn update_pokemon_stat(&self, pokemon_stat_id: &str, pokemon_stat_update_data: PokemonStatUpdateData) -> Result<PokemonStat, Error> {
        let updated_stat = diesel::update(pokemon_stats::dsl::pokemon_stats
            .filter(pokemon_stats::dsl::id.eq(&pokemon_stat_id)))
            .set(pokemon_stat_update_data)
            .get_result::<PokemonStat>(&mut self.pg_pool.connection()?)?;
        Ok(updated_stat)
    }

    /// Update a Pokemon type.
    async fn update_pokemon_type(&self, pokemon_type_id: &str, pokemon_type_update_data: PokemonTypeUpdateData) -> Result<PokemonType, Error> {
        let updated_type = diesel::update(pokemon_types::dsl::pokemon_types
            .filter(pokemon_types::dsl::id.eq(&pokemon_type_id)))
            .set(pokemon_type_update_data)
            .get_result::<PokemonType>(&mut self.pg_pool.connection()?)?;
        Ok(updated_type)
    }
}
