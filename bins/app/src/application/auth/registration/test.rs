use super::data::UserRequestData;
use actix_web::{web, ResponseError, http::StatusCode};
use crate::application::auth::registration::data::UserValidationRequestData;
use validr::*;

#[allow(dead_code)]
async fn test_actix_route_handler_user_attributes(
    test: web::Json<UserRequestData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[allow(dead_code)]
async fn test_actix_route_handler_user_validation(
    test: web::Json<UserValidationRequestData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_empty_email_field(){
    let data = UserRequestData{
        email: None,
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("Example_Password1211".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_empty_first_name_field(){
    let data = UserRequestData{
        email: Some("example.email@gmail.com".to_string()),
        first_name: None,
        last_name: Some("example_last_name".to_string()),
        password: Some("Example_Password1211".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_empty_last_name_field(){
    let data = UserRequestData{
        email: Some("example.email@gmail.com".to_string()),
        first_name: Some("example_name".to_string()),
        last_name: None,
        password: Some("jajajaja_Password1211".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_empty_password_field(){
    let data = UserRequestData{
        email: Some("example.email@gmail.com".to_string()),
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: None
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_password_eight_chars_long(){
    let data = UserRequestData{
        email: Some("example.email@gmail.com".to_string()),
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some(":O".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_register_user_failed_validation_weak_password(){
    let data = UserRequestData{
        email: Some("example.email@gmail.com".to_string()),
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("00000000000".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_resend_email_verification_failed_validation_empty_email_field(){
    let data = UserValidationRequestData{
        email: None,
        password: Some("pretty_GOOD_Password1211+".to_string()),
    };
    let response = test_actix_route_handler_user_validation(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[actix_web::main]
#[test]
async fn test_request_resend_email_verification_failed_validation_empty_password_field(){
    let data = UserValidationRequestData{
        email: Some("example.email@gmailcom".to_string()),
        password: None,
    };
    let response = test_actix_route_handler_user_validation(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}