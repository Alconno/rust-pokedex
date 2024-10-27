use diesel::{AsChangeset, Queryable};
use infrastructure::{DbConnection, schema::users};
use serde::{Deserialize, Serialize};
use error::Error;

use diesel::prelude::*;

use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub role: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    // Retrieve a user by email address from the database
    pub fn get_by_email(email: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::email.eq(email))
            .first(connection)
            .map_err(Error::from)
    }

    // Retrieve a user by email and password from the database
    pub fn get_by_email_and_password(email: &str, password: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::email.eq(email))
            .filter(users::password.eq(password))
            .first(connection)
            .map_err(Error::from)
    }

    // Retrieve a user by ID from the database
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::id.eq(id))
            .first(connection)
            .map_err(Error::from)
    }

    // Retrieve a user by refresh token from the database
    pub fn get_by_refresh_token(refresh_token: &str, connection: &mut DbConnection) -> Result<User, Error> {
        users::table
            .filter(users::refresh_token.eq(refresh_token))
            .first(connection)
            .map_err(Error::from)
    }

    // Put method to update whole user
    pub fn update_user(user: &User, connection: &mut DbConnection) -> Result<User, Error> {
        let updated_user = diesel::update(users::dsl::users
            .filter(users::dsl::id.eq(&user.id)))
            .set(user)
            .get_result::<User>(connection)
            .map_err(Error::from)?;
    
        Ok(updated_user)
    }

    // Patch method to set user refresh_token to None ( Logout user )
    pub fn remove_user_refresh_token(user_id: &str, connection: &mut DbConnection) -> Result<User, Error> {
        let updated_user = diesel::update(users::dsl::users
            .filter(users::dsl::id.eq(user_id)))
            .set(users::dsl::refresh_token.eq(None::<String>))
            .get_result::<User>(connection)
            .map_err(Error::from)?;
    
        Ok(updated_user)
    }

    // Patch method to set deleted_at date ( Deactivate/Soft Delete User )
    pub fn set_user_deleted_at_date(user_id: &str, connection: &mut DbConnection) -> Result<User, Error> {
        let updated_user = diesel::update(users::dsl::users
            .filter(users::dsl::id.eq(user_id)))
            .set(users::dsl::deleted_at.eq(Some(Utc::now().naive_utc())))
            .get_result::<User>(connection)
            .map_err(Error::from)?;
    
        Ok(updated_user)
    }

     // Patch method to set deleted_at to None ( Activate User )
     pub fn remove_user_deleted_at(user_id: &str, connection: &mut DbConnection) -> Result<User, Error> {
        let updated_user = diesel::update(users::dsl::users
            .filter(users::dsl::id.eq(user_id)))
            .set(users::dsl::deleted_at.eq(None::<NaiveDateTime>))
            .get_result::<User>(connection)
            .map_err(Error::from)?;
    
        Ok(updated_user)
    }

}