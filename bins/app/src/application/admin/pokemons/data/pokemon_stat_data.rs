use diesel::{prelude::Insertable, query_builder::AsChangeset};
use infrastructure::schema::pokemon_stats;
use serde::{Deserialize, Serialize};
use validr::{modifier_trim, Modifier, Rule, Validation};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PokemonStatDisplayData{
    pub id: String,
    pub name: Option<String>,
    pub base_stat: Option<i32>, 
    pub effort: Option<i32>,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestPokemonStatUpdateData{
    pub name: Option<String>,
    pub base_stat: Option<i32>,
    pub effort: Option<i32>,
}

impl Validation for RequestPokemonStatUpdateData{
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(name),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        let mut rules = Vec::new();

        if let Some(base_stat) = self.base_stat {
            rules.push(Rule::new("base_stat", move |_obj, error| {
                if base_stat <= 0 {
                    error.add("base_stat must be greater than 0");
                }
            }));
        }

        if let Some(effort) = self.effort {
            rules.push(Rule::new("effort", move |_obj, error| {
                if effort <= 0 {
                    error.add("effort must be greater than 0");
                }
            }));
        }
        rules
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = pokemon_stats)]
#[diesel(treat_none_as_null = true)]
pub struct PokemonStatUpdateData {
    pub name: String,
    pub base_stat: i32,
    pub effort: i32,
}

impl RequestPokemonStatUpdateData {
    pub fn insertable(self) -> PokemonStatUpdateData  {
        let name = self.name.unwrap_or("".to_string());
        let base_stat = self.base_stat.unwrap_or(0);
        let effort = self.effort.unwrap_or(0);
        PokemonStatUpdateData{
            name, base_stat, effort
        }
    }
}


impl PokemonStatUpdateData {
    pub fn merge(&mut self, other: &Self) {
        if self.name.is_empty() {
            self.name = other.name.clone();
        }
        if self.base_stat == 0 {
            self.base_stat = other.base_stat;
        }
        if self.effort == 0 {
            self.effort = other.effort;
        }
    }
}