use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse};
use error::Error;
use serde_json::json;
use support::helpers::{http::extract_token_by_name, verify_access_token::extract_user_id_from_access_token};
use validr::Validation;
use recaptcha_verify::{RecaptchaError, verify};
use crate::application::auth::login::{contract::LoginContract, data::UserLoginValidationRequestData};

pub async fn handle_login<T: LoginContract>(
    service: web::Data<T>,
    data: web::Json<UserLoginValidationRequestData>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {

    // Check if the user is already logged in - First check if token exists and then check if token's user exists
    let access_token = extract_token_by_name(&req, "access_token");
    if !access_token.is_empty(){
        if service.check_user_login(&(
            match extract_user_id_from_access_token(&access_token){
                Ok(extracted_user_id) => extracted_user_id,
                Err(e) => return Err(e),
            }
        )).await{
            return Ok(HttpResponse::Ok().json("User already logged in"));
        }
    }
  
    // Validate user input data
    let data = data.into_inner().validate()?.insertable();

    println!("validating captcha");
    // Validate reCAPTCHA
    let remote_ip = Some(req.peer_addr().unwrap().ip());
    let res:Result<(), RecaptchaError> = verify(&config::get_default("RECAPTCHA_LEGACY_KEY", ""), &data.recaptcha_response, remote_ip.as_ref()).await;
    println!("{:?}", res);
    match res.is_ok(){
        true=>println!("pass"),
        false => {
            println!("fail");
            return Err(Error::BadRequest("Captcha fail".to_string()));
        },
    }

    // Attempt to login the user
    let jwt_tokens = service.login(data).await?;

    // Construct tokens JSON
    let tokens_json = json!({
        "access_token": jwt_tokens.access_token,
        "refresh_token": jwt_tokens.refresh_token,
    });

    // Construct tokens cookie
    let tokens_cookie = Cookie::build("tokens", tokens_json.to_string())
        .path("/")
        .finish();
    
    // Return HTTP response with tokens and cookie
    Ok(HttpResponse::Ok().cookie(tokens_cookie).json(jwt_tokens))
}
