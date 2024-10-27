use crate::store::models::pokemons::api_pokemon::ApiAbilities;
use crate::store::models::pokemons::pokemon_ability::{PokemonAbility, DatabasePokemonAbility};
use infrastructure::DbConnection;
use error::Error;

pub async fn create(
    abilities: Vec<ApiAbilities>, 
    pokemon_id: &str, 
    connection: &mut DbConnection
) -> Result<(), Error> {
    for attr in abilities {
        let name = attr.ability.as_ref().and_then(|a| a.name.as_ref()).map(|val| val.to_string()).unwrap();
        let exists_in_db = match PokemonAbility::get_by_pokemon_id_and_name(&pokemon_id, Some(&name), connection).await {
            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => false,
            Ok(_) | Err(_) => true,
        };

        if !exists_in_db {
            let entity = DatabasePokemonAbility { name: Some(name), pokemon_id: pokemon_id.to_string() };
            PokemonAbility::create(entity, connection)?;
        }
    }
    Ok(())
}