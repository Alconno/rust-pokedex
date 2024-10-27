use diesel::{prelude::Insertable, query_builder::AsChangeset};
use infrastructure::schema::pokemon_types;
use serde::{Deserialize, Serialize};
use validr::{modifier_trim, Modifier, Rule, Validation};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PokemonTypeDisplayData{
    pub id: String,
    pub name: Option<String>,
    pub slot: Option<i32>,
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestPokemonTypeUpdateData{
    pub name: Option<String>,
    pub slot: Option<i32>,
}


impl Validation for RequestPokemonTypeUpdateData{
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(name),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        let mut rules = Vec::new();

        if let Some(slot) = self.slot {
            rules.push(Rule::new("slot", move |_obj, error| {
                if slot <= 0 {
                    error.add("slot must be greater than 0");
                }
            }));
        }

        rules
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = pokemon_types)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonTypeUpdateData {
    pub name: String,
    pub slot: i32,
}

impl RequestPokemonTypeUpdateData {
    pub fn insertable(self) -> PokemonTypeUpdateData  {
        let name = self.name.unwrap_or("".to_string());
        let slot = self.slot.unwrap_or(0);
        PokemonTypeUpdateData{
            name, slot
        }
    }
}

impl PokemonTypeUpdateData {
    pub fn merge(&mut self, other: &Self) {
        if self.name == "" {
            self.name = other.name.clone();
        }
        if self.slot == 0 {
            self.slot = other.slot;
        }
    }
}
