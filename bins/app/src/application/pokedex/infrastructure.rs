use std::sync::Arc;
use super::{contract::{PgRepositoryContract, PgServiceContract}, data::PokemonAttributes};
use async_trait::async_trait;
use error::Error;
use infrastructure::{db::Pg, schema::{pokemons, user_pokedexes}};
use support::store::models::pokemons::pokemon::Pokemon;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, TextExpressionMethods};

pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn get_paginated_user_pokemons(&self, user_id: &str, pokemon_attributes: PokemonAttributes) -> Result<Vec<Pokemon>, Error> {
        // Build the base query
        let mut query = pokemons::table
            .inner_join(user_pokedexes::table.on(pokemons::id.eq(user_pokedexes::pokemon_id)))
            .filter(user_pokedexes::user_id.eq(user_id))
            .select(pokemons::all_columns)
            .into_boxed();

        // Apply search to the query
        if !pokemon_attributes.search.is_empty(){
            query = query.filter(pokemons::name.like(format!("%{}%", &pokemon_attributes.search)));
        }

        // Apply sorting to the query
        query = match &*pokemon_attributes.sort_by {
            "created_at" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::created_at.asc()),
                _ => query.order(pokemons::created_at.desc()),
            },
            "updated_at" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::updated_at.asc()),
                _ => query.order(pokemons::updated_at.desc()),
            },
            "name" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::name.asc()),
                _ => query.order(pokemons::name.desc()),
            },
            "base_experience" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::base_experience.asc()),
                _ => query.order(pokemons::base_experience.desc()),
            },
            "height" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::height.asc()),
                _ => query.order(pokemons::height.desc()),
            },
            "weight" => match &*pokemon_attributes.sort {
                "ASC" => query.order(pokemons::weight.asc()),
                _ => query.order(pokemons::weight.desc()),
            },
            _ => query,
        };
    
        // Apply pagination and return result
        match query
            .offset((pokemon_attributes.page - 1) * pokemon_attributes.per_page)
            .limit(pokemon_attributes.per_page)
            .load::<Pokemon>(&mut self.pg_pool.connection()?)
        {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::InternalError(format!(
                "Failed fetching paginated pokemons from user's pokedex. Error: {}",
                e.to_string()
            ))),
        }
    }
    
}    

pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {

}
