use diesel::{prelude::Insertable, query_builder::AsChangeset};
use infrastructure::schema::pokemon_abilities;
use serde::{Deserialize, Serialize};
use validr::{modifier_trim, Modifier, Validation};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PokemonAbilityDisplayData{
    pub id: String,
    pub name: Option<String>, 
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestPokemonAbilityUpdateData{
    pub name: Option<String>
}

impl Validation for RequestPokemonAbilityUpdateData{
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(name),
        ]
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = pokemon_abilities)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonAbilityUpdateData {
    pub name: String
}

impl RequestPokemonAbilityUpdateData {
    pub fn insertable(self) -> PokemonAbilityUpdateData  {
        let name = self.name.unwrap();
        PokemonAbilityUpdateData{
            name,
        }
    }
}


impl PokemonAbilityUpdateData {
    pub fn merge(&mut self, other: &Self) {
        if self.name.is_empty() {
            self.name = other.name.clone();
        }
    }
}