use std::sync::Arc;

use super::{contract::{PgRepositoryContract, PgServiceContract}, data::UserAttemptData};
use async_trait::async_trait;
use error::Error;
use infrastructure::{db::Pg, schema::{user_attempts, user_pokedexes}};
use support::store::models::{pokemons::pokemon::Pokemon, user_attempts::UserAttempt, user_pokedex::{UserPokedex, UserPokedexData}};
use diesel::{ExpressionMethods, RunQueryDsl};

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Get the blacklisted Pokemon IDs for a user.
    async fn get_blacklist(&self, user_id: &str) -> Result<Vec<i32>, Error>{
        let catches = match UserPokedex::get_all_by_user_id(user_id, &mut self.pg_pool.connection()?){
            Ok(catches) => catches,
            Err(e) => return Err(
                Error::InternalError(format!("Failed fetching User Pokedex catches when getting blacklist: ({})", e.to_string()))
            )
        };
        
        let mut result: Vec<i32> = Vec::new();
        for catch in catches{
            let pokemon = match Pokemon::get_by_id(&catch.pokemon_id, &mut self.pg_pool.connection()?).await{
                Ok(pokemon) => pokemon,
                Err(e) => return Err(
                    Error::NotFoundWithCause(format!("Failed fetching pokemon by id when getting blacklist. Error: ({})", e.to_string()))
                ),
            };
            if let Some(poke_id) = pokemon.pokemon_id{
                result.push(poke_id);
            }
        }
        Ok(result)
    }

    /// Get a Pokemon by its ID.
    async fn get_pokemon_by_pokemon_id(&self, pokemon_id: i32) -> Result<Pokemon, Error> {
        let mut connection = self.pg_pool.connection()?;
        match Pokemon::get_by_pokemon_id(pokemon_id, &mut connection).await{
            Ok(pokemon) => Ok(pokemon),
            Err(e) => Err(
                Error::NotFoundWithCause(format!("Failed fetching pokemon by id. Error: ({})", e.to_string()))
            )
        }
    }

    /// Get the attempt by its ID and user ID.
    async fn get_attempt_by_user_id_and_pokemon_id(&self, user_id: &str, pokemon_id: &str) -> Result<UserAttempt, Error>{
        let mut connection = self.pg_pool.connection()?;
        match UserAttempt::get_by_user_id_and_pokemon_id(user_id, pokemon_id, &mut connection).await{
            Ok(user_attempt) => Ok(user_attempt),
            Err(e) => Err(
                Error::NotFoundWithCause(
                    format!("Failed fetching user attempt by user_id and pokemon_id. Error: ({})", e.to_string())
                )
            )
        }
    }

    /// Check an attempt by its ID and user ID.
    async fn check_attempt(&self, attempt_id: &str) -> Result<UserAttempt, Error>{
        let mut connection = self.pg_pool.connection()?;
        match UserAttempt::get_by_id(attempt_id, &mut connection).await{
            Ok(user_attempt) => Ok(user_attempt),
            Err(e) => Err(
                Error::NotFoundWithCause(
                    format!("Failed fetching user attempt by attempt_id and user_id. Error: ({})", e.to_string())
                )
            )
        }
    }

    /// Get a Pokemon by its ID.
    async fn get_pokemon_by_id(&self, pokemon_id: &str) -> Result<Pokemon, Error>{
        let mut connection = self.pg_pool.connection()?;
        match Pokemon::get_by_id(pokemon_id, &mut connection).await{
            Ok(pokemon) => Ok(pokemon),
            Err(e) => Err(
                Error::NotFoundWithCause(format!("Failed fetching pokemon by pokemon_id. Error: ({})", e.to_string()))
            )
        }
    }
}

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    /// Create a new attempt.
    async fn create_new_attempt(&self, pokemon_id: &str, authenticated_user_id: &str) -> Result<UserAttempt, Error> {
        let mut connection = self.pg_pool.connection()?;
        let data = UserAttemptData::new(authenticated_user_id, pokemon_id);
        diesel::insert_into(user_attempts::table)
            .values(data)
            .get_result::<UserAttempt>(&mut connection)
            .map_err(Error::from)
    }

    /// Create a Pokemon in the Pokedex.
    async fn create_pokemon_in_pokedex(&self, user_id: &str, pokemon_id: &str) -> Result<(), Error>{
        let mut connection = self.pg_pool.connection()?;
        let data = UserPokedexData::new(user_id, pokemon_id);
        diesel::insert_into(user_pokedexes::table)
            .values(data)
            .get_result::<UserPokedex>(&mut connection)?;
        Ok(())
    }

    /// Update an attempt to successful.
    async fn update_attempt_to_successful(&self, attempt_id: &str) -> Result<(), Error>{
        let mut connection = self.pg_pool.connection()?;
        diesel::update(user_attempts::table)
            .filter(user_attempts::id.eq(attempt_id))
            .set(user_attempts::is_successful.eq(true))
            .execute(&mut connection)?;
        Ok(())
    }
}
