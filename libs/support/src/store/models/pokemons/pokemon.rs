use diesel::Queryable;
use infrastructure::{DbConnection, schema::pokemons};
use crate::store::models::pokemons::api_pokemon::ApiPokemon;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use error::Error;



#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, AsChangeset)]
#[diesel(table_name = pokemons)]
#[diesel(treat_none_as_null = true)]
pub struct Pokemon{
    pub id: String,
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub pokemon_id: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub image: Option<String>,
    pub weight: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Pokemon{
    pub async fn get_by_pokemon_id(pokemon_id: i32, connection: &mut DbConnection) -> Result<Pokemon, Error>{
        pokemons::table
            .filter(pokemons::pokemon_id.eq(pokemon_id))
            .first(connection)
            .map_err(Error::from)
    }

    pub async fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<Pokemon, Error>{
        pokemons::table
            .filter(pokemons::id.eq(id))
            .first(connection)
            .map_err(Error::from)
    }

    pub fn create(data: DatabasePokemon, connection: &mut DbConnection) -> Result<Pokemon, Error>{
        diesel::insert_into(pokemons::table)
            .values(data)
            .get_result::<Pokemon>(connection)
            .map_err(error::Error::from)
    }

    pub fn get_all_pokemons(connection: &mut DbConnection) -> Result<Vec<Pokemon>, Error>{
        pokemons::table
            .get_results::<Pokemon>(connection)
            .map_err(Error::from)
    }

    pub fn update_pokemon(pokemon: &Pokemon, connection: &mut DbConnection) -> Result<Pokemon, Error> {
        let updated_pokemon = diesel::update(pokemons::dsl::pokemons
            .filter(pokemons::dsl::id.eq(&pokemon.id)))
            .set(pokemon)
            .get_result::<Pokemon>(connection)
            .map_err(Error::from)?;
    
        Ok(updated_pokemon)
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = pokemons)]
pub struct DatabasePokemon {
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub pokemon_id: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub image: Option<String>,
}

impl From<ApiPokemon> for DatabasePokemon {
    fn from(api_pokemon: ApiPokemon) -> Self {
        DatabasePokemon {
            name: api_pokemon.name,
            base_experience: api_pokemon.base_experience,
            height: api_pokemon.height,
            pokemon_id: api_pokemon.id,
            is_default: api_pokemon.is_default,
            order: api_pokemon.order,
            weight: api_pokemon.weight,
            image: api_pokemon.sprites.unwrap().front_default,
        }
    }
}
