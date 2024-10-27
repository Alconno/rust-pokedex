use actix_web::{ web, HttpResponse};
use infrastructure::db::{Pg, Rd};
use std::sync::Arc;
use crate::middleware::Auth;

pub fn configure(pg_pool: Arc<Pg>, _rd_pool: Arc<Rd>, cfg: &mut web::ServiceConfig) {

    // Routes
    cfg.service(
        web::resource("/_auth_health")
            .route(web::get().to(|| async {HttpResponse::Ok().body("I am auth healthy!")}))
            .wrap(Auth),
    );

    cfg.route(
        "/_health",
        web::get().to(|| async { HttpResponse::Ok().body("I am healthy!")})
    );

    // Docs
    add_docs_routes(cfg);

    // Auth routes
    add_auth_routes(pg_pool.clone(), cfg);

    // Pokedex routes
    add_pokedex_routes(pg_pool.clone(), cfg);

    // Admin routes
    add_admin_routes(pg_pool.clone(), cfg);
}

// auth routes
fn add_auth_routes(p: Arc<Pg>, c: &mut web::ServiceConfig) {
    crate::application::auth::registration::setup::routes(p.clone(), c);
    crate::application::auth::login::setup::routes(p, c);
}

// pokedex routes
fn add_pokedex_routes(p: Arc<Pg>, c: &mut web::ServiceConfig){
    crate::application::pokemon_game::setup::routes(p.clone(), c);
    crate::application::pokedex::setup::routes(p, c);
}

// admin routes
fn add_admin_routes(p: Arc<Pg>, c: &mut web::ServiceConfig){
    crate::application::admin::users::setup::routes(p.clone(), c);
    crate::application::admin::pokemons::setup::routes(p.clone(), c);
    crate::application::admin::pokedexes::setup::routes(p.clone(), c);
}

// docs
fn add_docs_routes(cfg: &mut web::ServiceConfig){
    // Define all the dev routes
    if &config::get_default("IS_DEV", "false") == "true" {
        // Display openapi specs in yaml
        cfg.route(
            "/openapi.yaml",
            web::get()
                .to(|| async { HttpResponse::Ok().body(include_str!("../../../../openapi.yaml")) }),
        );
        // Display openapi specs in json
        cfg.route(
            "/openapi.json",
            web::get().to(|| async {
                HttpResponse::Ok()
                    .insert_header(("Content-Type", "application/json"))
                    .body(include_str!("../../../../openapi.json"))
            }),
        );
    }
}