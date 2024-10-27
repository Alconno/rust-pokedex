use super::{contract::{PgRepositoryContract, PgServiceContract, PokemonGameContract}, data::ResponseData};
use async_trait::async_trait;
use constants::global_constants::POKEMON_RANGE;
use error::Error;
use support::helpers::random_pokemon_id::pick_from_blacklist;

pub struct PokemonGame<A: PgRepositoryContract, B: PgServiceContract> {
    pub repository: A,
    pub service: B,
}


#[async_trait]
impl<A, B> PokemonGameContract for PokemonGame<A, B>
where
    A: PgRepositoryContract + Sync + Send,
    B: PgServiceContract + Sync + Send,
{
    /// Get a random Pokemon for a user.
    async fn get_random_pokemon(&self, user_id: &str) -> Result<ResponseData, Error> {
        let blacklist = self.repository.get_blacklist(user_id).await?;
        if blacklist.len() as i32 == POKEMON_RANGE{
            return Err(Error::AllPokemonsCollected("All pokemons collected".to_string()))
        }
        
        let random_pokemon_id = pick_from_blacklist(POKEMON_RANGE+1, blacklist).await;
        
        // Retrieve random pokemon from the database
        let random_pokemon = self.repository.get_pokemon_by_pokemon_id(random_pokemon_id).await?;
        
        // Handle attempts
        let attempt = match self.repository.get_attempt_by_user_id_and_pokemon_id(&user_id, &random_pokemon.id).await {
            Ok(attempt) => attempt,
            Err(Error::NotFoundWithCause(_)) => {
                self.service.create_new_attempt(&random_pokemon.id, user_id).await?
            }
            Err(e) => return Err(e),
        };
        
        // Return attempt id and pokemon image
        Ok(ResponseData::new(
            random_pokemon.image.unwrap_or("Image Not Found".to_string()), 
            attempt.id, 
            random_pokemon_id
        ))
    }


    /// Check a user's guess for a Pokemon.
    async fn check_guess(&self, guess: &str, user_id: &str, attempt_id: &str) -> Result<(), Error> {
        let attempt = self.repository.check_attempt(attempt_id).await?;

        if attempt.is_successful {
            return Err(Error::BadRequest("Pokemon already in user's pokedex".to_string()));
        }

        let pokemon = self.repository.get_pokemon_by_id(&attempt.pokemon_id).await?;
        let pokemon_name = pokemon.name
            .ok_or_else(|| Error::InternalError("Pokemon name not found".to_string()))?;

        if guess != pokemon_name {
            return Err(Error::WrongPokemonGuess("Wrong pokemon guess".to_string()));
        }

        // Add pokemon to user's collection
        self.service.create_pokemon_in_pokedex(user_id, &pokemon.id).await?;
        // Set current attempt as successful
        self.service.update_attempt_to_successful(attempt_id).await?;
 
        Ok(())
    }
    
}
