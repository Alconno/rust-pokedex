
use serde::{Deserialize, Serialize};
use validr::{modifier_trim, rule_email, rule_required, Modifier, Rule, Validation};

#[derive(Serialize, Deserialize)]
pub struct CaptchaResponse{
    event: CaptchaEvent
}

#[derive(Serialize, Deserialize)]
pub struct CaptchaEvent{
    token: String,
    expected_action: String,
    site_key: String,
}

// Used to validate user login
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserLoginValidationRequestData {
    pub email: Option<String>,
    pub password: Option<String>,
    pub recaptcha_response: Option<String>,
}

impl Validation for UserLoginValidationRequestData {
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_trim!(email),
            modifier_trim!(password),
            modifier_trim!(recaptcha_response),
        ]
    }

    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(password),
            rule_required!(recaptcha_response),
        ]
    }
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct UserLoginValidationData {
    pub email: String,
    pub password: String,
    pub recaptcha_response: String,
}

impl UserLoginValidationRequestData{
    pub fn insertable(self) -> UserLoginValidationData{
        let email = self.email.unwrap();
        let password = self.password.unwrap();
        let recaptcha_response = self.recaptcha_response.unwrap();
        UserLoginValidationData{
            email,
            password,
            recaptcha_response,
        }
    }
}