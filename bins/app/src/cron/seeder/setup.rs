use std::sync::Arc;
use crony::Job;
use infrastructure::db::Pg;
use super::infrastructure::Seeder;
use super::domain::SeederJob;


pub fn job(
    pg_pool: Arc<Pg>,
) -> Box<dyn Job>{
    let service = SeederJob{
        seeder: Seeder{
            pg: pg_pool.clone()
        }
    };

    Box::new(service)
}
