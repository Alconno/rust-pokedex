use actix_web::{web, http::StatusCode};
use validr::Validation;

use super::data::RequestPokedexAttributes;

// Test handler for Actix route
#[allow(dead_code)]
async fn test_actix_route_handler_pokedex_pagination(
    test: web::Query<RequestPokedexAttributes>,
) -> StatusCode {
    match test.into_inner().validate() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

// Define the default values for RequestPokedexAttributes
impl Default for RequestPokedexAttributes {
    fn default() -> Self {
        RequestPokedexAttributes {
            sort_by: None,
            sort: None,
            page: None,
            per_page: None,
        }
    }
}

// Test cases for user pagination
#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_passed_with_no_data() {
    let data = RequestPokedexAttributes::default();
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_passed_with_only_page() {
    let data = RequestPokedexAttributes {
        page: Some(1),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_failed_with_negative_page() {
    let data = RequestPokedexAttributes {
        page: Some(-5),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::BAD_REQUEST);
}

#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_failed_with_negative_page_size() {
    let data = RequestPokedexAttributes {
        per_page: Some(-25),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::BAD_REQUEST);
}

#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_passed_with_only_page_size() {
    let data = RequestPokedexAttributes {
        per_page: Some(2),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokedex_pagination_passed_with_all_fields() {
    let data = RequestPokedexAttributes {
        page: Some(1),
        per_page: Some(5),
        sort_by: Some("created_at".to_string()),
        sort: Some("DESC".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokedex_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}
