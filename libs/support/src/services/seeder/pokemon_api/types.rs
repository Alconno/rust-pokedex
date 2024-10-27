use crate::store::models::pokemons::api_pokemon::ApiTypes;
use crate::store::models::pokemons::pokemon_type::{PokemonType, DatabasePokemonType};
use infrastructure::DbConnection;
use error::Error;

pub async fn create(
    types: Vec<ApiTypes>, 
    pokemon_id: &str, 
    connection: &mut DbConnection
) -> Result<(), Error> {
    for attr in types {
        let name = attr.type_.as_ref().and_then(|a| a.name.as_ref()).map(|val| val.to_string()).unwrap();
        let exists_in_db = match PokemonType::get_by_pokemon_id_and_name(&pokemon_id, Some(&name), connection).await {
            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => false,
            Ok(_) | Err(_) => true,
        };

        if !exists_in_db {
            let entity = DatabasePokemonType { name:Some(name), slot: attr.slot, pokemon_id: pokemon_id.to_string() };
            PokemonType::create(entity, connection)?;
        }
    }
    Ok(())
}
