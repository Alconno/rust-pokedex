use std::sync::Arc;
use crony::Runner;
use infrastructure::db::Pg;

pub mod seeder;

pub fn create_cron(pg: Arc<Pg>) -> Runner {
    Runner::new()
        .add(seeder::setup::job(pg.clone()))
}
