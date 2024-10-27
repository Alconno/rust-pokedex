use diesel::Queryable;
use infrastructure::{DbConnection, schema::pokemon_abilities};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use error::Error;


#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, AsChangeset)]
#[diesel(table_name = pokemon_abilities)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonAbility {
    pub id: String,
    pub pokemon_id: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PokemonAbility{
    pub async fn get_by_pokemon_id_and_name(pokemon_id: &str, name: Option<&str>, connection: &mut DbConnection) -> Result<PokemonAbility, Error>{
        pokemon_abilities::table
            .filter(pokemon_abilities::pokemon_id.eq(pokemon_id))
            .filter(pokemon_abilities::name.eq(name))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<PokemonAbility, Error>{
        pokemon_abilities::table
            .filter(pokemon_abilities::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_all_by_pokemon_id(pokemon_id: &str, connection: &mut DbConnection) -> Result<Vec<PokemonAbility>, Error> {
        pokemon_abilities::table
            .filter(pokemon_abilities::pokemon_id.eq(pokemon_id))
            .get_results::<PokemonAbility>(connection)
            .map_err(Error::from)
    }
    

    pub fn create(data: DatabasePokemonAbility, connection: &mut DbConnection) -> Result<PokemonAbility, Error>{
        diesel::insert_into(pokemon_abilities::table)
            .values(data)
            .get_result::<PokemonAbility>(connection)
            .map_err(Error::from)
    }

    pub fn update(ability_id: &str, ability: PokemonAbility, connection: &mut DbConnection) -> Result<PokemonAbility, Error>{
        let updated_ability = diesel::update(pokemon_abilities::dsl::pokemon_abilities
            .filter(pokemon_abilities::dsl::id.eq(ability_id)))
            .set(ability)
            .get_result::<PokemonAbility>(connection)?;
        Ok(updated_ability)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = pokemon_abilities)]
pub struct DatabasePokemonAbility {
    pub name: Option<String>,
    pub pokemon_id: String,
}
