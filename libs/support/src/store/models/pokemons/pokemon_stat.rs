use diesel::Queryable;
use infrastructure::{DbConnection, schema::pokemon_stats};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use error::Error;

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[diesel(table_name = pokemon_stats)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonStat{
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub base_stat: Option<i32>,
    pub effort: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonStat{
    pub async fn get_by_pokemon_id_and_name(pokemon_id: &str, name: Option<&str>, connection: &mut DbConnection) -> Result<PokemonStat, Error>{
        pokemon_stats::table
            .filter(pokemon_stats::pokemon_id.eq(pokemon_id))
            .filter(pokemon_stats::name.eq(name))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<PokemonStat, Error>{
        pokemon_stats::table
            .filter(pokemon_stats::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_all_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<Vec<PokemonStat>, Error> {
        pokemon_stats::table
            .filter(pokemon_stats::pokemon_id.eq(pokemon_id))
            .get_results::<PokemonStat>(connection)
            .map_err(Error::from)
    }

    pub fn create(data: DatabasePokemonStat, connection: &mut DbConnection) -> Result<PokemonStat, Error>{
        diesel::insert_into(pokemon_stats::table)
            .values(data)
            .get_result::<PokemonStat>(connection)
            .map_err(Error::from)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = pokemon_stats)]
pub struct DatabasePokemonStat {
    pub name: Option<String>,
    pub base_stat: Option<i32>,
    pub effort: Option<i32>,
    pub pokemon_id: String,
}
