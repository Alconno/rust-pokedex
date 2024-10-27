
use std::pin::Pin;
use std::future::Future;
use super::contract::{SeederContract, SeederJobContract};
use crony::Job;
use error::Error;
use tokio::runtime::Runtime;

pub struct SeederJob<T: SeederContract> {
    pub seeder: T,
}

impl<T> SeederJobContract for SeederJob<T>
where
    T: SeederContract + Send + Sync,
{
    fn handle_job<'life0, 'async_trait>(&'life0 self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
    {
        Box::pin(async move {
            self.seeder.seed().await?;
            Ok(())
        })
    }
}

impl<T> Job for SeederJob<T>
where
    T: SeederContract + Send + Sync,
{
    fn handle(&self) {
        if let Err(error) = Runtime::new()
            .expect("Error creating tokio runtime")
            .block_on(self.handle_job())
        {
            error!("{}", error);
        }
    }

    // Run once a day at 7 AM
    fn schedule(&self) -> crony::Schedule {
        "0 0 7 * * *".parse().expect("Error parsing cron schedule")
    }
    
}