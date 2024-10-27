use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url::extract_uuid_after_keyword;
use crate::application::admin::users::contract::AdminUsersContract;

pub async fn handle_user_logout<T: AdminUsersContract>(
    service: web::Data<T>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {

    // Extract user_id from url
    let user_id = extract_uuid_after_keyword(&req, "users").unwrap_or("".to_string());
    
    // Call the logout method on the service
    let user = service.logout(&user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}
