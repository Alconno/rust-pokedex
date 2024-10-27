use std::cmp;
use crate::{
    services::seeder::pokemon_api::{
        abilities, stats, types
    },
    store::models::pokemons::{
        api_pokemon::ApiPokemon, 
        pokemon::Pokemon, 
        pokemon_ability::PokemonAbility, 
        pokemon_stat::PokemonStat, 
        pokemon_type::PokemonType
    }
};
use infrastructure::DbConnection;
use error::Error;


pub async fn create_pokemons(connection: &mut DbConnection, starting_index:i32, amount:i32, do_api_call_regardless_of_db_data: bool) -> Result<(String, ()), Error> {
    let index = cmp::min(starting_index, 1);

    // Checks if pokemons exist in specified range before doing an api call 
    // ( this DOES NOT confirm all attributes 100% exist, just that pokemons exist and they own some attributes)
    if !do_api_call_regardless_of_db_data {
        return check_db_data(index, amount, connection).await;
    }

   
    // Doing API call on all pokemons regardless of their existence in database
    let api_pokemons = ApiPokemon::get_pokemons_from_api(&index, &amount).await?;

    // Adding them to db ( of course duplicated are avoided )
    for api_pokemon in api_pokemons {
        add_pokemon_to_db(api_pokemon, connection).await?;
    }

    Ok(("- All missing pokemons have been seeded.".to_string(), ()))
}

/// Checks and updates data of pokemons in the database
pub async fn check_db_data(starting_index: i32, amount: i32, connection: &mut DbConnection) -> Result<(String, ()), Error>{
    let mut response_message = String::from(" - All Pokemons already seeded.");
    let mut missing_pokemons: Vec<i32> = Vec::new();

    for pokemon_id in starting_index..=amount {
        let success = match Pokemon::get_by_pokemon_id(pokemon_id, connection).await {
            Ok(pokemon) => {
                let attributes_exist = 
                    PokemonAbility::get_by_pokemon_id(&pokemon.id,  connection).await.is_ok() &&
                    PokemonType::get_by_pokemon_id(&pokemon.id,  connection).await.is_ok() &&
                    PokemonStat::get_by_pokemon_id(&pokemon.id,  connection).await.is_ok();
                

                if !attributes_exist {
                    let api_pokemon = ApiPokemon::get_specific_pokemon_from_api(pokemon_id).await?;
                    create_attributes(api_pokemon, &pokemon.id, connection).await?;
                    response_message = String::from(" - Pokemon attributes not found. Successfully added them.");
                }
                true
            }
            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => {
                let api_pokemon = ApiPokemon::get_specific_pokemon_from_api(pokemon_id).await?;
                add_pokemon_to_db(api_pokemon, connection).await?;
                missing_pokemons.push(pokemon_id);
                true
            }
            Err(_) => false
        };

        if !success {
            return Err(Error::NotFoundWithCause("pokemon creation failed".to_string()));
        }
    }

    if !missing_pokemons.is_empty() {
        response_message = String::from(" - Pokemons not found. Successfully added them: ");
        response_message += &missing_pokemons.iter()
            .map(|pokemon| format!(" {},", pokemon))
            .collect::<String>();
    }

    Ok((response_message, ()))
}



/// Adds a Pokemon to the database.
pub async fn add_pokemon_to_db(api_pokemon: ApiPokemon, connection: &mut DbConnection) -> Result<(), Error>{
    if let Some(pokemon_id) = api_pokemon.id {
        match Pokemon::get_by_pokemon_id(pokemon_id, connection).await {
            Ok(pokemon) => create_attributes(api_pokemon, &pokemon.id, connection).await?,

            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => {
                let new_pokemon = Pokemon::create(api_pokemon.clone().into(), connection)?;
                create_attributes(api_pokemon, &new_pokemon.id, connection).await?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}



/// Creates attributes for a Pokemon in the database
pub async fn create_attributes(api_pokemon: ApiPokemon, pokemon_id: &str, connection: &mut DbConnection) -> Result<(), Error> {
    abilities::create(api_pokemon.abilities, &pokemon_id, connection).await?;
    stats::create(api_pokemon.stats, &pokemon_id, connection).await?;
    types::create(api_pokemon.types, &pokemon_id, connection).await?;
    Ok(())
}
