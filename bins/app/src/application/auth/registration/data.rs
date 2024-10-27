use infrastructure::schema::{action_tokens, users};
use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable};
use validr::*;


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserRequestData{
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
}

impl Validation for UserRequestData{
    fn modifiers(&self)->Vec<Modifier<Self>>{
        vec![
            modifier_trim!(email),
            modifier_trim!(first_name),
            modifier_trim!(last_name),
            modifier_trim!(password),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>>{
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(first_name),
            rule_required!(last_name),
            rule_required!(password),
            rule_length_min!(password,8),
            Rule::new("password", |obj: &Self, error|{
                if let Some(v) = &obj.password{
                    if support::helpers::validations::password_strength(v){
                        error.add("weak_password");
                    }
                }
            }),
        ]
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
pub struct UserData {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl UserRequestData {
    pub fn insertable(self) -> UserData  {
        let email = self.email.unwrap();
        let first_name = self.first_name.unwrap();
        let last_name = self.last_name.unwrap();
        let password = bcrypt::hash(self.password.unwrap(), 10).unwrap();
        UserData{
            email,
            first_name,
            last_name,
            password
        }
    }
}




#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = action_tokens)]
#[diesel(treat_none_as_null = true)]
pub struct ActionTokenData {
    pub entity_id: String,
    pub action_name: String,
    pub token: String
}






// Used to validate email token resending to user
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserValidationRequestData {
    pub email: Option<String>,
    pub password: Option<String>
}

impl Validation for UserValidationRequestData {
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(email),
            modifier_trim!(password),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(password),
        ]
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserValidationData {
    pub email: String,
    pub password: String,
}

impl UserValidationRequestData{
    pub fn insertable(self) -> UserValidationData{
        let email = self.email.unwrap();
        let password = self.password.unwrap();
        UserValidationData{
            email,
            password
        }
    }
}