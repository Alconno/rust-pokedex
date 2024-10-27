use diesel::{Queryable, AsChangeset};
use error::Error;
use infrastructure::{DbConnection, schema::action_tokens};
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use chrono::{NaiveDateTime, Utc};

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    AsChangeset,
)]
#[diesel(table_name = action_tokens)]
#[diesel(treat_none_as_null = true)]
pub struct ActionToken {
    pub id: String,
    pub entity_id: String,
    pub token: String,
    pub action_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub executed_at: Option<NaiveDateTime>
}

impl ActionToken {
    // helper function for get action token by token column
    pub fn get_by_token(
        token: &str,
        connection: &mut DbConnection
    ) -> Result<ActionToken, error::Error> {
        action_tokens::table
        .filter(action_tokens::token.eq(token)) 
        .first(connection)
        .map_err(error::Error::from)
    }

    pub async fn action_token_update_executed_at(
        token_id: &str, 
        connection: &mut DbConnection
    ) -> Result<usize, Error> {
        diesel::update(action_tokens::table)
        .filter(action_tokens::id.eq(token_id))
        .set(action_tokens::executed_at.eq(Utc::now()))
        .execute(connection)
        .map_err(Error::from)
    }
}