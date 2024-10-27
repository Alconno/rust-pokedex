use actix_web::HttpResponse;
use error::Error;
use support::helpers::obfuscator::obfuscate;
use tera::{Context, Tera};

pub async fn index() -> Result<HttpResponse, Error> {
    // Retrieve the RECAPTCHA_KEY from the environment variables
    let recaptcha_key = config::get_default("RECAPTCHA_KEY", "");

    // Create a Tera context and insert the RECAPTCHA_KEY into it
    let mut context = Context::new();
    context.insert("recaptcha_key", &recaptcha_key);

    // Initialize the Tera template engine
    let tera = Tera::new("bins/app/src/application/templates/*").unwrap();

    // Render the HTML template with the context
    let rendered_html = tera.render("login.html", &context)
        .map_err(|e| Error::InternalError(e.to_string()))?;

    // Return the HTML or obfuscated HTML as the HTTP response
    Ok(HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(match config::get_default("FRONT_OBFUSCATION", "")=="true"{
        true => obfuscate(&rendered_html).await?,
        false => rendered_html,
    }))
}
