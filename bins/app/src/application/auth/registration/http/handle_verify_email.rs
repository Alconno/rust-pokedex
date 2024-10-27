use crate::application::auth::registration::contract::RegisterContract;
use actix_web::{web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::http::part_from_path;

pub async fn handle_verify_email<T: RegisterContract>(
    service: web::Data<T>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let action_token = part_from_path::<String>(&request, "action_token")?;
    
    let action_token = service.check_action_token(&action_token).await?;

    service.verify_email(action_token).await?;

    // Send user to login after successful email verification
    Ok(HttpResponse::Found().append_header(("Location", "/auth/login")).finish())
}
