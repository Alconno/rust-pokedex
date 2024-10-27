pub use diesel::pg::PgConnection;
use diesel::Connection;
use std::env;
pub use r2d2_redis::redis::{Client, ConnectionAddr, ConnectionInfo, IntoConnectionInfo};
use crate::{DbPool, DbConnection, RedisPool, RedisConnection};

/// Struct to hold our postgres pool with some integrated commands
#[derive(Clone)]
pub struct Pg {
    pool: DbPool,
}

impl Pg {
    /// Create new instance of this struct with integrated pool
    pub fn new(pool: DbPool) -> Self {
        Pg { pool }
    }

    /// Get connection from the pool
    pub fn connection(&self) -> Result<DbConnection, error::Error> {
        self.pool.get().map_err(error::Error::from)
    }

    /// Staticly generates completely new independent connection
    /// to use somewhere where pool cannot be passed.
    pub fn single_connection() -> PgConnection {
        let database_url = env::var("PG_DB_URL").expect("PG_DB_URL must be set");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
    }
}


/// Struct to hold redis database pool
#[derive(Clone)]
pub struct Rd {
    pool: RedisPool,
}

impl Rd {
    /// Create new instance of this struct with integrated pool
    pub fn new(pool: RedisPool) -> Self {
        Rd { pool }
    }

    /// Get connection from the pool
    pub fn connection(&self) -> RedisConnection {
        match self.pool.get() {
            Ok(connection) => connection,
            Err(e) => panic!("{}", e),
        }
    }

    /// Staticly generates completely new independent connection
    /// to use somewhere where pool cannot be passed.
    pub fn single_connection() -> Client {
        match Client::open(get_rd_connection_info()) {
            Ok(connection) => connection,
            Err(e) => panic!("Error getting single redis connection: {e}"),
        }
    }
}

/// Generate connection info for redis
pub fn get_rd_connection_info() -> ConnectionInfo {
    let mut params = config::get_multiple_default(vec![("RD_DB_URL", ""), ("RD_DB_PASSWORD", "")]);

    let password: String = params.pop().unwrap().parse().unwrap();
    let database_url = params.pop().unwrap();
    assert!(!database_url.is_empty(), "RD_DB_URL must be set");

    let mut connection = database_url.into_connection_info().unwrap();

    if !password.is_empty() {
        connection.passwd = Some(password);
    }

    connection
}