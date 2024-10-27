use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtTokens {
    pub refresh_token: String,
    pub access_token: String,
}
impl JwtTokens {
    // Implement a constructor to create a new instance of JwtTokens
    pub fn new(refresh_token: String, access_token: String) -> Self {
        Self {
            refresh_token,
            access_token
        }
    }
}