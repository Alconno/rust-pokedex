pub mod web;
mod cron;
use log::error;
mod application;
mod middleware;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main(){
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    config::initialize().await;

    let (state, pg, rd) = infrastructure::state::initialize();

    match support::services::seeder::seed(pg.clone()).await {
        Ok(()) => (),
        Err(e) => error!("Error seeding db: {:?}", e),
    }

    let _cron = cron::create_cron(pg.clone()).run();

    web::start_server(state, pg, rd).await.expect("Error while starting/running http server");
}
