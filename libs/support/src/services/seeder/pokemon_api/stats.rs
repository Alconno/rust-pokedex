use crate::store::models::pokemons::api_pokemon::ApiStats;
use crate::store::models::pokemons::pokemon_stat::{PokemonStat, DatabasePokemonStat};
use infrastructure::DbConnection;
use error::Error;

pub async fn create(
    stats: Vec<ApiStats>, 
    pokemon_id: &str, 
    connection: &mut DbConnection
) -> Result<(), Error> {
    for attr in stats {
        let name = attr.stat.as_ref().and_then(|a| a.name.as_ref()).map(|val| val.to_string()).unwrap();
        let exists_in_db = match PokemonStat::get_by_pokemon_id_and_name(&pokemon_id, Some(&name), connection).await {
            Err(Error::Diesel(ref cause)) if cause.to_string() == "Record not found" => false,
            Ok(_) | Err(_) => true,
        };

        if !exists_in_db {
            let entity = DatabasePokemonStat { 
                name: Some(name), 
                base_stat: attr.base_stat, 
                effort: attr.effort, 
                pokemon_id: pokemon_id.to_string(),
            };
            PokemonStat::create(entity, connection)?;
        }
    }
    Ok(())
}