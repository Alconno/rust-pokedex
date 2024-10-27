use async_trait::async_trait;
use error::Error;

#[async_trait]
pub trait SeederContract {
    async fn seed(&self) -> Result<(), Error>;
}

#[async_trait]
pub trait SeederJobContract {
    async fn handle_job(&self) -> Result<(), Error>;
}