use actix_web::{web, ResponseError, http::StatusCode};
use validr::*;

use super::data::UserLoginValidationRequestData;

#[allow(dead_code)]
async fn test_actix_route_handler_login_attributes(
    test: web::Json<UserLoginValidationRequestData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_empty_required_field_email() {
    let data = UserLoginValidationRequestData {
        email: None,
        password: Some("12345Abcdefgh.".to_string()),
        recaptcha_response: Some("yes".to_string()),
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_field_email_format() {
    let data = UserLoginValidationRequestData {
        email: Some("example_bad_email".to_string()),
        password: Some("pass123yes+4.".to_string()),
        recaptcha_response: Some("yes".to_string()),
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_empty_required_field_password() {
    let data = UserLoginValidationRequestData {
        email: Some("any.mail@gmail.com".to_string()),
        password: None,
        recaptcha_response: Some("yes".to_string()),
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}
