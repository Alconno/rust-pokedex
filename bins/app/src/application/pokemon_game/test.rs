use actix_web::{web, ResponseError, http::StatusCode};
use validr::*;

use super::data::RequestAttemptData;


#[allow(dead_code)]
async fn test_actix_route_handler_user_attributes(
    test: web::Json<RequestAttemptData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_guess_pokemon_failed_validation_empty_required_field_guess() {
    let data = RequestAttemptData {
        guess: None,
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY)
}
