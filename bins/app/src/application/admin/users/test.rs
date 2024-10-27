use actix_web::{web, http::StatusCode};
use validr::Validation;
use super::data::RequestUserAttributes;

// Test handler for Actix route
#[allow(dead_code)]
async fn test_actix_route_handler_user_pagination(
    test: web::Query<RequestUserAttributes>,
) -> StatusCode {
    match test.into_inner().validate() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

// Define the default values for RequestUserAttributes
impl Default for RequestUserAttributes {
    fn default() -> Self {
        RequestUserAttributes {
            search: None,
            sort_by: None,
            sort: None,
            page: None,
            per_page: None,
            hide_staff: None,
            hide_deleted: None,
        }
    }
}

// Test cases for user pagination
#[actix_web::main]
#[test]
async fn test_admin_user_pagination_passed_with_no_data() {
    let data = RequestUserAttributes::default();
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_user_pagination_passed_with_only_page() {
    let data = RequestUserAttributes {
        page: Some(1),
        ..Default::default()
    };
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_user_pagination_failed_with_negative_page() {
    let data = RequestUserAttributes {
        page: Some(-5),
        ..Default::default()
    };
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::BAD_REQUEST);
}

#[actix_web::main]
#[test]
async fn test_admin_user_pagination_failed_with_negative_page_size() {
    let data = RequestUserAttributes {
        per_page: Some(-25),
        ..Default::default()
    };
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::BAD_REQUEST);
}

#[actix_web::main]
#[test]
async fn test_admin_user_pagination_passed_with_only_page_size() {
    let data = RequestUserAttributes {
        per_page: Some(2),
        ..Default::default()
    };
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_user_pagination_passed_with_all_fields() {
    let data = RequestUserAttributes {
        page: Some(1),
        per_page: Some(5),
        sort_by: Some("first_name".to_string()),
        sort: Some("DESC".to_string()),
        hide_staff: Some(false),
        hide_deleted: Some(false),
        ..Default::default()
    };
    let response = test_actix_route_handler_user_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}
