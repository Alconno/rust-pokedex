pub use diesel::pg::PgConnection;
use crate::{DbPool, RedisPool};
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2_redis::RedisConnectionManager;


//**************************************************/
//*                    PG
//**************************************************/

// Startup and return Db pool
pub fn get_pg_pool() -> DbPool {
    let mut params =
        config::get_multiple_default(vec![("PG_DB_URL", ""), ("PG_POOL_MAX_SIZE", "8")]);

    let pool_size: u32 = params.pop().unwrap().parse().unwrap();
    let database_url = params.pop().unwrap();
    assert!(!database_url.is_empty(), "PG_DB_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create postgres db pool: {e}"))
}



//**************************************************/
//*                    REDIS
//**************************************************/

/// Startup and return redis pool
pub fn get_rd_pool() -> RedisPool {
    let pool_size: u32 = config::get_default("RD_DB_POOL_MAX_SIZE", "8")
        .parse()
        .unwrap();

    let connection = crate::db::get_rd_connection_info();
    let manager = RedisConnectionManager::new(connection)
        .unwrap_or_else(|e| panic!("Error connecting to redis {e}"));

    r2d2::Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .unwrap_or_else(|e| panic!("Failed to create redis db pool: {e}"))
}
