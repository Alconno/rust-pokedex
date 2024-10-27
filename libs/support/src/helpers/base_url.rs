use actix_web::HttpRequest;
use error::Error;

/// Get base_url
/// if DEV env get base_url from Origin header
pub fn get_base_url(req: &HttpRequest) -> String {
    let mut base_url = get_header_origin(req);

    if base_url.ends_with('/') {
        base_url.pop();
    }

    base_url
}

/// Get origin header from request or if missing, return from config
pub fn get_header_origin(req: &HttpRequest) -> String {
    let mut base_url = match part_from_header(req, "host") {
        Ok(header) => header,
        Err(_) => config::get_default("WEBSITE_BASE_URL", ""),
    };

    if base_url.ends_with('/') {
        base_url.pop();
    }

    base_url
}

/// Helper function that will get given parameter easily from the request header
pub fn part_from_header(req: &HttpRequest, name: &str) -> Result<String, Error> {
    let value = match req.headers().get(name) {
        Some(val) => match val.to_str().ok() {
            Some(v) => v,
            None => return Err(Error::BadRequest(format!("attribute_missing:{}", name))),
        },
        None => return Err(Error::BadRequest(format!("attribute_missing:{}", name))),
    };

    Ok(value.to_string())
}

/// Helper function to extract uuid after /keyword/ in url
pub fn extract_uuid_after_keyword(req: &HttpRequest, keyword: &str) -> Option<String> {
    // Get the request path
    let path = req.path();

    let keyword_path = format!("/{}/", keyword);

    // Find the index of "/keyword/"
    if let Some(keyword_index) = path.find(&keyword_path) {
        // Get the substring after "/keyword/"
        let substring = &path[keyword_index + keyword_path.len()..];

        // Extract the UUID substring
        if let Some(uuid_substring) = substring.split('/').next() {
            // Check if the substring is a valid UUID
            if uuid::Uuid::parse_str(uuid_substring).is_ok() {
                return Some(uuid_substring.to_string());
            }
        }
    }

    None
}