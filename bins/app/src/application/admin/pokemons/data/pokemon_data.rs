use diesel::{prelude::Insertable, query_builder::AsChangeset};
use infrastructure::schema::pokemons;
use serde::{Deserialize, Serialize};
use support::store::models::pokemons::pokemon::Pokemon;
use validr::{modifier_trim, Modifier, Rule, Validation};

use super::{pokemon_ability_data::PokemonAbilityDisplayData, pokemon_stat_data::PokemonStatDisplayData, pokemon_type_data::PokemonTypeDisplayData};



// Pokemon Models for display
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PokemonShowData{
    pub pokemon: Pokemon,
    pub attributes: PokemonAttributesDisplayData,
}

// Pokemon Models for update
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestPokemonUpdateData{
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub weight: Option<i32>,
}


impl Validation for RequestPokemonUpdateData{
    fn modifiers(&self)->Vec<Modifier<Self>>{
        vec![
            modifier_trim!(name)
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        let mut rules = Vec::new();

        if let Some(base_experience) = self.base_experience {
            rules.push(Rule::new("base_experience", move |_obj, error| {
                if base_experience <= 0 {
                    error.add("base_experience must be greater than 0");
                }
            }));
        }

        if let Some(height) = self.height {
            rules.push(Rule::new("height", move |_obj, error| {
                if height <= 0 {
                    error.add("height must be greater than 0");
                }
            }));
        }

        if let Some(weight) = self.weight {
            rules.push(Rule::new("weight", move |_obj, error| {
                if weight <= 0 {
                    error.add("weight must be greater than 0");
                }
            }));
        }

        rules
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = pokemons)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonUpdateData {
    pub name: String,
    pub base_experience: i32,
    pub height: i32,
    pub weight: i32,
}

impl RequestPokemonUpdateData {
    pub fn insertable(self) -> PokemonUpdateData  {
        let name = self.name.unwrap();
        let base_experience = self.base_experience.unwrap();
        let height = self.height.unwrap();
        let weight = self.weight.unwrap();
        PokemonUpdateData{
            name, base_experience, height, weight
        }
    }
}






// Pokemon Attribute Models for display
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PokemonAttributesDisplayData{
    pub abilities: Vec<PokemonAbilityDisplayData>,
    pub stats: Vec<PokemonStatDisplayData>,
    pub types: Vec<PokemonTypeDisplayData>
}