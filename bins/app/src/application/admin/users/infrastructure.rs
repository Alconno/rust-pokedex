use std::sync::Arc;
use async_trait::async_trait;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use error::Error;
use infrastructure::{db::Pg, schema::users::{self}};
use super::{contract::{PgRepositoryContract, PgServiceContract}, data::UserAttributes};
use support::store::models::user::User;



pub struct PgRepository {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Get a user by ID.
    async fn get_by_id(&self, user_id: &str) -> Result<User, Error> {
        Ok(match User::get_by_id(user_id, &mut self.pg_pool.connection()?) {
            Ok(user) => user,
            Err(e) => return Err(Error::NotFoundWithCause(format!("User not found. Error: {}", e))),
        })
    }

    /// Get paginated users.
    async fn paginated(&self, user_attributes: UserAttributes) -> Result<Vec<User>, Error> {
        // Build the base query
        let mut query = users::table.into_boxed();
    
        // Apply filtering to the query
        if user_attributes.hide_staff {
            query = query.filter(users::role.eq("User"));
        }
        if user_attributes.hide_deleted {
            query = query.filter(users::deleted_at.is_null());
        }

        // Apply search to the query
        if !user_attributes.search.is_empty(){
            query = query.filter(users::first_name.like(format!("%{}%", &user_attributes.search))
                .or(users::email.like(format!("%{}%", &user_attributes.search))));
        }

        // Apply sorting to the query
        query = match &*user_attributes.sort_by {
            "created_at" => match &*user_attributes.sort {
                "ASC" => query.order(users::created_at.asc()),
                _ => query.order(users::created_at.desc()),
            },
            "updated_at" => match &*user_attributes.sort {
                "ASC" => query.order(users::updated_at.asc()),
                _ => query.order(users::updated_at.desc()),
            },
            "deleted_at" => match &*user_attributes.sort {
                "ASC" => query.order(users::deleted_at.asc()),
                _ => query.order(users::deleted_at.desc()),
            },
            "email_verified_at" => match &*user_attributes.sort {
                "ASC" => query.order(users::email_verified_at.asc()),
                _ => query.order(users::email_verified_at.desc()),
            },
            "first_name" => match &*user_attributes.sort {
                "ASC" => query.order(users::first_name.asc()),
                _ => query.order(users::first_name.desc()),
            },
            "last_name" => match &*user_attributes.sort {
                "ASC" => query.order(users::last_name.asc()),
                _ => query.order(users::last_name.desc()),
            },
            _ => query,
        };

        // Apply pagination and return result
        match query
            .offset((user_attributes.page - 1) * user_attributes.per_page)
            .limit(user_attributes.per_page)
            .load::<User>(&mut self.pg_pool.connection()?) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::InternalError(format!("Failed fetching pokedexes for pagination. Error: {}", e.to_string()))),
        }
    }
}



pub struct PgService {
    pub pg_pool: Arc<Pg>,
}

#[async_trait]
impl PgServiceContract for PgService {
    // Patch method to set user refresh_token to None ( Logout user )
    async fn logout(&self, user_id: &str) -> Result<User, Error> {
        Ok(User::remove_user_refresh_token(user_id, &mut self.pg_pool.connection()?)?)
    }

    // Patch method to set deleted_at date ( Deactivate/Soft Delete User )
    async fn deactivate(&self, user_id: &str) -> Result<User, Error> {
        Ok(User::set_user_deleted_at_date(user_id, &mut self.pg_pool.connection()?)?)
    }

    // Patch method to set deleted_at to None ( Activate User )
    async fn activate(&self, user_id: &str) -> Result<User, Error> {
        Ok(User::remove_user_deleted_at(user_id, &mut self.pg_pool.connection()?)?)
    }
}