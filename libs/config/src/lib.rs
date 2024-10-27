pub use dotenv::{from_path, vars};
use error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn is_dev() -> bool {
    get_default("IS_DEV", "false") == *"true"
}

#[allow(dead_code)]
fn print_env() {
    let env_vars = std::env::vars();
    for (key, value) in env_vars {
        println!("{key} = {value:?}");
    }
}

/// Initialize global configurations from dotenv
pub async fn initialize() {
    env();
    dotenv(None);
    args();
    vault().await;
    set_defaults();
    println!("Configuration loaded and ready.");
    // print_env();
}

/// Set and replace existing key if found
pub fn set(key: &str, value: &str) {
    let temp = get(key);

    if temp.is_ok() {
        std::env::remove_var(key);
    }

    std::env::set_var(key, value)
}

/// Set the value only if previous values doesn't exist on its place
pub fn set_if_not_empty(key: &str, value: &str) {
    let temp = get(key);

    if temp.is_err() {
        std::env::set_var(key, value)
    }
}

/// Get configuration variable
pub fn get(key: &str) -> Result<String, Error> {
    let key = String::from(key);

    std::env::var(key).map_err(|_e| Error::NotFound)
}

/// Get mutliple keys at once back
pub fn get_multiple(keys: Vec<&str>) -> Result<Vec<String>, Error> {
    let mut collected = vec![];

    for key in keys.iter() {
        let value = get(key)?;

        collected.push(value);
    }

    Ok(collected)
}

/// Get value from config or pass the default value
pub fn get_default(key: &str, default: &str) -> String {
    get(key).unwrap_or_else(|_| String::from(default))
}

/// Get mutliple keys with set default values
pub fn get_multiple_default(keys: Vec<(&str, &str)>) -> Vec<String> {
    let mut collected = vec![];

    for (key, default) in keys.iter() {
        collected.push(get_default(key, default));
    }

    collected
}

/// Set default configurations for the application
pub fn set_defaults() {
    // Cookie settings
    set_if_not_empty("COOKIE_NAME", "pokedex");
    set_if_not_empty("COOKIE_DOMAIN", "localhost");
    set_if_not_empty("COOKIE_SAME_SITE", "lax");
    set_if_not_empty("COOKIE_SECURE", "false");
    set_if_not_empty("COOKIE_HTTP", "false");
    set_if_not_empty("COOKIE_PATH", "/");

    // Vault configurations
    set_if_not_empty("VAULT_ADDR", "");
    set_if_not_empty("VAULT_TOKEN", "");
    set_if_not_empty("VAULT_MODULE_WORKSPACE", "");

    set_if_not_empty("EMAIL_SEND_VIA", "mandrill");
    set_if_not_empty("DEFAULT_SENDER_NAME", "Pokedex");

    set_if_not_empty("WEBSITE_BASE_URL", "");
    set_if_not_empty("API_BASE_URL", "");
}

/// Load values from dotenv and attach it to values
pub fn dotenv(path: Option<String>) {
    match path {
        Some(p) => match from_path(&p) {
            Ok(_) => (),
            Err(e) => panic!("Couldn't load the dotenv config at '{p}', error: {e}"),
        },
        None => {
            dotenv::dotenv().ok();
        }
    };
}

/// Load values from shell environment
pub fn env() {
    let vars: Vec<(String, String)> = std::env::vars().collect();

    for (key, value) in vars.iter() {
        set(key, value);
    }
}

/// Load values from command line arguments passed
pub fn args() {
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        let mut items = arg.split('=');

        if let Some(key) = items.next() {
            let value = items.next().unwrap_or("true");
            set(key, value);
        }
    }
}

/// Load values from vault instance if any configured
pub async fn load_vault() -> BTreeMap<String, String> {
    let mut params = match get_multiple(vec!["VAULT_ADDR", "VAULT_TOKEN", "VAULT_MODULE_WORKSPACE"])
    {
        Ok(params) => params,
        Err(_e) => return BTreeMap::new(),
    };

    if params.len() != 3 {
        return BTreeMap::new();
    }

    let workspace = params.pop().unwrap();
    let token = params.pop().unwrap();
    let address = params.pop().unwrap();

    if workspace.is_empty() || token.is_empty() || address.is_empty() {
        return BTreeMap::new();
    }

    println!("Reading vault...");
    get_vault_secret(&address, &token, &workspace).await.data
}

async fn vault() {
    let data = load_vault().await;
    if data.is_empty() {
        return;
    }

    println!("Loading vault...");
    for (key, value) in data {
        // When tests are executed on gitlab, vault values overwrite variables like DATABASE_URL
        if cfg!(feature = "dev") {
            set_if_not_empty(&key, &value);
        } else {
            set(&key, &value);
        }
    }
}

/// Make HTTP request to vault and retrieve the secrets
async fn get_vault_secret(address: &str, token: &str, workspace: &str) -> VaultSecretDataResponse {
    let mut clean_address = address.to_string();
    let mut clean_workspace = workspace.to_string();

    if clean_address.ends_with('/') {
        let _ = clean_address.pop();
    }

    if clean_workspace.starts_with('/') {
        let mut temp_clean_workspace: String = clean_workspace.chars().rev().collect();
        let _ = temp_clean_workspace.pop();
        clean_workspace = temp_clean_workspace.chars().rev().collect();
    }

    let url = format!("{0}/v1/{1}", &clean_address, &clean_workspace);

    let response = match reqwest::Client::new()
        .get(&url)
        .header(
            reqwest::header::HeaderName::from_static("x-vault-token"),
            reqwest::header::HeaderValue::from_str(token).expect("Vault token is invalid"),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            panic!(
                "Errored while trying to send request to Vault: {} -- {:#?}",
                &url, e
            );
        }
    };

    match response.json::<Value>().await {
        Ok(json_obj) => {
            if let Ok(stringified) = serde_json::to_string(&json_obj) {
                match serde_json::from_str::<VaultSecretResponse>(&stringified) {
                    Ok(response) => response.data,
                    Err(e) => panic!("Could not parse Vault response into VaultSecretResponse, recieved response {json_obj:#?}, error when parsing {e:#?} "),
                }
            } else {
                panic!("Could not stringify recieved json {json_obj:#?}");
            }
        }
        Err(e) => {
            panic!("Error while parsing Valut response into generic json: {e:#?}");
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct VaultSecretDataResponse {
    data: BTreeMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct VaultSecretResponse {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: i64,
    data: VaultSecretDataResponse,
}

