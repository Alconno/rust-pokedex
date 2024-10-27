use diesel::Queryable;
use infrastructure::{DbConnection, schema::pokemon_types};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use error::Error;


#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[diesel(table_name = pokemon_types)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonType{
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub slot: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonType{
    pub async fn get_by_pokemon_id_and_name(pokemon_id: &str, name: Option<&str>, connection: &mut DbConnection) -> Result<PokemonType, Error>{
        pokemon_types::table
            .filter(pokemon_types::pokemon_id.eq(pokemon_id))
            .filter(pokemon_types::name.eq(name))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<PokemonType, Error>{
        pokemon_types::table
            .filter(pokemon_types::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_all_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<Vec<PokemonType>, Error> {
        pokemon_types::table
            .filter(pokemon_types::pokemon_id.eq(pokemon_id))
            .get_results::<PokemonType>(connection)
            .map_err(Error::from)
    }

    pub fn create(data: DatabasePokemonType, connection: &mut DbConnection) -> Result<PokemonType, Error>{
        diesel::insert_into(pokemon_types::table)
            .values(data)
            .get_result::<PokemonType>(connection)
            .map_err(Error::from)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = pokemon_types)]
pub struct DatabasePokemonType {
    pub name: Option<String>,
    pub slot: Option<i32>,
    pub pokemon_id: String,
}
