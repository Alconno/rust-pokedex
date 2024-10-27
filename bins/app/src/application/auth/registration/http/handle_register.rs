use crate::application::auth::registration::{
    contract::RegisterContract, data::UserRequestData
};
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url;
use validr::Validation;


pub async fn handle_register<T: RegisterContract>(
    service: web::Data<T>,
    data: web::Json<UserRequestData>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let base_url = base_url::get_base_url(&request);
    let data = data.into_inner().validate()?.insertable();

    let action_token = service.register(data, &base_url).await?;

    let mut response = HttpResponse::Created().finish();

    // If IS_DEV is true set email_token to response header for testing purpose
    if &config::get_default("IS_DEV", "")[..] == "true" {
        response.headers_mut().insert(
            header::HeaderName::from_static("action_token"),
            header::HeaderValue::from_str(&action_token.token)
            .expect("Failed to create header value"),
        );
    }

    Ok(response)
}
