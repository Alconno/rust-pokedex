use actix_web::{web, ResponseError, http::StatusCode};
use validr::Validation;

use super::data::{pokemon_ability_data::RequestPokemonAbilityUpdateData, pokemon_data::RequestPokemonUpdateData, pokemon_pagination_data::RequestPokemonAttributes, pokemon_stat_data::RequestPokemonStatUpdateData, pokemon_type_data::RequestPokemonTypeUpdateData};



#[allow(dead_code)]
async fn test_actix_route_handler_pokemon_pagination(
    test: web::Query<RequestPokemonAttributes>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

// Define the default values for RequestPokemonAttributes
impl Default for RequestPokemonAttributes {
    fn default() -> Self {
        RequestPokemonAttributes {
            search: None,
            sort_by: None,
            sort: None,
            page: None,
            per_page: None,
        }
    }
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_no_data() {
    let data = RequestPokemonAttributes::default();
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_page() {
    let data = RequestPokemonAttributes {
        page: Some(1),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_failed_with_negative_page() {
    let data = RequestPokemonAttributes {
        page: Some(-5),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_failed_with_negative_per_page() {
    let data = RequestPokemonAttributes {
        per_page: Some(-25),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_per_page() {
    let data = RequestPokemonAttributes {
        per_page: Some(25),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_sort_by() {
    let data = RequestPokemonAttributes {
        sort_by: Some("created_at".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_sort_asc() {
    let data = RequestPokemonAttributes {
        sort: Some("asc".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_sort_desc() {
    let data = RequestPokemonAttributes {
        sort: Some("desc".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_failed_with_only_sort() {
    let data = RequestPokemonAttributes {
        sort: Some("traktor_marko".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}


#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_only_search() {
    let data = RequestPokemonAttributes {
        search: Some("weed".to_string()),
        ..Default::default()
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_pagination_passed_with_all_fields() {
    let data = RequestPokemonAttributes {
        search: Some("weed".to_string()),
        sort_by: Some("created_at".to_string()),
        sort: Some("desc".to_string()),
        page: Some(1),
        per_page: Some(5),
    };
    let response = test_actix_route_handler_pokemon_pagination(web::Query(data)).await;
    assert_eq!(response, StatusCode::OK);
}













#[allow(dead_code)]
fn test_actix_route_handler_pokemon_update(
    test: web::Json<RequestPokemonUpdateData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_passed_update_only_name() {
    let data = RequestPokemonUpdateData {
        name: Some("new_name".to_string()),
        base_experience: None,
        height: None,
        weight: None,
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_passed_update_only_base_experience() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: Some(15),
        height: None,
        weight: None,
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::OK);
}


#[actix_web::main]
#[test]
async fn test_admin_pokemon_passed_update_only_height() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: None,
        height: Some(15),
        weight: None,
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_passed_update_only_weight() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: None,
        height: None,
        weight: Some(15),
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_passed_update_all_fields() {
    let data = RequestPokemonUpdateData {
        name: Some("yes_name".to_string()),
        base_experience: Some(15),
        height: Some(16),
        weight: Some(17),
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::OK);
}


#[actix_web::main]
#[test]
async fn test_admin_pokemon_failed_update_base_experience_zero_or_less() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: Some(0),
        height: None,
        weight: None,
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_failed_update_height_zero_or_less() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: None,
        height: Some(0),
        weight: None,
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}


#[actix_web::main]
#[test]
async fn test_admin_pokemon_failed_update_weight_zero_or_less() {
    let data = RequestPokemonUpdateData {
        name: None,
        base_experience: None,
        height: None,
        weight: Some(0),
    };
    let response = test_actix_route_handler_pokemon_update(web::Json(data));
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}







#[allow(dead_code)]
async fn test_actix_route_handler_pokemon_ability_update(
    test: web::Json<RequestPokemonAbilityUpdateData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_ability_passed_update_only_name() {
    let data = RequestPokemonAbilityUpdateData {
        name: Some("new_name".to_string()),
    };
    let response = test_actix_route_handler_pokemon_ability_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_ability_passed_update_no_name() {
    let data = RequestPokemonAbilityUpdateData {
        name: None,
    };
    let response = test_actix_route_handler_pokemon_ability_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}








#[allow(dead_code)]
async fn test_actix_route_handler_pokemon_stat_update(
    test: web::Json<RequestPokemonStatUpdateData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_passed_update_name_only() {
    let data = RequestPokemonStatUpdateData {
        name: Some("new_name".to_string()),
        base_stat: None,
        effort: None,
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_passed_update_base_stat_only() {
    let data = RequestPokemonStatUpdateData {
        name: None,
        base_stat: Some(5),
        effort: None,
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_passed_update_effort_only() {
    let data = RequestPokemonStatUpdateData {
        name: None,
        base_stat: None,
        effort: Some(9),
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_passed_update_all_fields() {
    let data = RequestPokemonStatUpdateData {
        name: Some("rname".to_string()),
        base_stat: Some(13),
        effort: Some(9),
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_failed_update_base_stat_zero_or_less() {
    let data = RequestPokemonStatUpdateData {
        name: None,
        base_stat: Some(0),
        effort: None,
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_stat_failed_update_effort_zero_or_less() {
    let data = RequestPokemonStatUpdateData {
        name: None,
        base_stat: None,
        effort: Some(0),
    };
    let response = test_actix_route_handler_pokemon_stat_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}







#[allow(dead_code)]
async fn test_actix_route_handler_pokemon_type_update(
    test: web::Json<RequestPokemonTypeUpdateData>,
) -> StatusCode{
    match test.into_inner().validate(){
        Ok(_) => StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_type_passed_update_name_only() {
    let data = RequestPokemonTypeUpdateData {
        name: Some("new_name".to_string()),
        slot: None,
    };
    let response = test_actix_route_handler_pokemon_type_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_type_passed_update_slot_only() {
    let data = RequestPokemonTypeUpdateData {
        name: None,
        slot: Some(17),
    };
    let response = test_actix_route_handler_pokemon_type_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_type_passed_update_all_fields() {
    let data = RequestPokemonTypeUpdateData {
        name: Some("rtype".to_string()),
        slot: Some(19),
    };
    let response = test_actix_route_handler_pokemon_type_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::OK);
}

#[actix_web::main]
#[test]
async fn test_admin_pokemon_type_failed_update_slot_zero_or_less() {
    let data = RequestPokemonTypeUpdateData {
        name: None,
        slot: Some(0),
    };
    let response = test_actix_route_handler_pokemon_type_update(web::Json(data)).await;
    assert_eq!(response, StatusCode::UNPROCESSABLE_ENTITY);
}