use super::contract::SeederContract;
use async_trait::async_trait;
use infrastructure::db::Pg;
use std::sync::Arc;
use error::Error;

#[derive(Clone)]
pub struct Seeder {
    pub pg: Arc<Pg>,
}

#[async_trait]
impl SeederContract for Seeder {
    async fn seed(&self) -> Result<(), Error> {
        match support::services::seeder::seed(self.pg.clone()).await {
            Ok(()) => (),
            Err(e) => error!("Error seeding db: {:?}", e),
        }
        Ok(())
    }
}