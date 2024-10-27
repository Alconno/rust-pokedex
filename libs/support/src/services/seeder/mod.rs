pub mod pokemon_api;


use constants::global_constants::POKEMON_RANGE;
use infrastructure::DbConnection;
use infrastructure::db::Pg;
use std::sync::Arc;
use error::Error;
use log::{error, info};

pub async fn seed(pg: Arc<Pg>) -> Result<(), Error>{
    info!("Starting seeding proccess");
    
    if &config::get_default("IS_DEV", "")[..] == "true"{
        match pg.connection(){
            Ok(connection) => run_dev(connection).await?,
            Err(e) => error!("Couldn't set database connection. {}", e),
        };
    }

    Ok(info!("Seeding completed."))
}

pub async fn run_dev(mut connection: DbConnection) -> Result<(), Error> {
    info!("Starting pokemon seeding:");
    match pokemon_api::create_pokemons(&mut connection, 1, POKEMON_RANGE, false).await {
        Ok((message, ())) => info!("{}", message),
        Err(e) => error!("Error seeding pokemons: {:?}", e),
    }
    Ok(())
}