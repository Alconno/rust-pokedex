use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use error::Error;
use infrastructure::{
    db::Pg, 
    schema::{user_attempts, user_pokedexes},
};
use support::store::models::{user_attempts::UserAttempt, user_pokedex::{UserPokedex, UserPokedexData}};

use super::{contract::{PgRepositoryContract, PgServiceContract}, data::PokedexAttributes};

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {

   /// Get paginated user pokedexes.
    async fn paginated(
        &self,
        pokedex_attributes: &PokedexAttributes,
    ) -> Result<Vec<UserPokedex>, Error> {
        // Build the base query
        let mut query = user_pokedexes::table.into_boxed();

         // Apply sorting to the query
         query = match &*pokedex_attributes.sort_by {
            "created_at" => match &*pokedex_attributes.sort {
                "ASC" => query.order(user_pokedexes::created_at.asc()),
                _ => query.order(user_pokedexes::created_at.desc()),
            },
            "updated_at" => match &*pokedex_attributes.sort {
                "ASC" => query.order(user_pokedexes::updated_at.asc()),
                _ => query.order(user_pokedexes::updated_at.desc()),
            },
            _ => query,
        };

        // Apply pagination and return result
        match query
            .offset((pokedex_attributes.page - 1) * pokedex_attributes.per_page)
            .limit(pokedex_attributes.per_page)
            .load::<UserPokedex>(&mut self.pg_pool.connection()?)
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::InternalError(format!(
                "Failed fetching user pokedexes for pagination. Error: {}",
                e.to_string()
            ))),
        }
    }

    /// Get user pokedex data by ID.
    async fn get_by_id(&self, user_pokedex_id: &str) -> Result<UserPokedexData, Error>{
        let user_pokedex_data: (String, String) = match user_pokedexes::table
            .select((user_pokedexes::user_id, user_pokedexes::pokemon_id))
            .filter(user_pokedexes::id.eq(user_pokedex_id))
            .first(&mut self.pg_pool.connection()?)
            .map_err(Error::from){
                Ok((user_id, pokemon_id)) => (user_id, pokemon_id),
                Err(e) => return Err(Error::NotFoundWithCause(format!("Failed fetching user pokedex data by id. Error: {}", e.to_string())))
            };
        Ok(UserPokedexData{ user_id: user_pokedex_data.0, pokemon_id: user_pokedex_data.1})
    }

    /// Get user attempt by user ID and pokemon ID.
    async fn get_user_attempt_by_user_id_and_pokemon_id(&self, user_id: &str, pokemon_id: &str) -> Result<UserAttempt, Error>{
        match UserAttempt::get_by_user_id_and_pokemon_id(user_id, pokemon_id, &mut self.pg_pool.connection()?).await{
            Ok(user_attempt) => Ok(user_attempt),
            Err(e) => Err(Error::NotFoundWithCause(format!("Failed fetching user attempt by user_id and pokemon_id. Error: {}", e.to_string())))
        }
    }
}

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    /// Delete user pokedex by ID.
    async fn delete(&self, user_pokedex_id: &str) -> Result<(), Error>{
        diesel::delete(user_pokedexes::table)
            .filter(user_pokedexes::id.eq(user_pokedex_id))
            .execute(&mut self.pg_pool.connection()?)
            .map_err(Error::from)?;
        Ok(())
    }

    /// Set attempt success to false by ID.
    async fn set_attempt_success_false(&self, user_attempt_id: &str) -> Result<(), Error>{
        diesel::update(user_attempts::table)
            .filter(user_attempts::id.eq(user_attempt_id))
            .set(user_attempts::is_successful.eq(false))
            .execute(&mut self.pg_pool.connection()?)
            .map_err(Error::from)?;
        Ok(())
    }
}
