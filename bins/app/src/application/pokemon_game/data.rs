use diesel::prelude::Insertable;
use infrastructure::schema::user_attempts;
use serde::{Deserialize, Serialize};
use validr::{modifier_lowercase, modifier_trim, rule_required, Modifier, Rule, Validation};



#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable)]
#[diesel(table_name = user_attempts)]
#[diesel(treat_none_as_null = true)]
pub struct UserAttemptData {
    pub user_id: String,
    pub pokemon_id: String,
}

impl UserAttemptData {
    pub fn new(user_id: &str, pokemon_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            pokemon_id: pokemon_id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ResponseData {
    pub pokemon_image: String,
    pub attempt_id: String,
    pub pokemon_id: i32,
}
impl ResponseData {
    pub fn new(pokemon_image: String, attempt_id: String, pokemon_id: i32) -> Self {
        Self {
            pokemon_image,
            attempt_id,
            pokemon_id,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RequestAttemptData {
    pub guess: Option<String>,
}

impl Validation for RequestAttemptData {
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(guess),
            modifier_lowercase!(guess)
            ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        vec![rule_required!(guess)]
    }
}