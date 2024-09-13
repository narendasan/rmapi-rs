use crate::error::Error;
use const_format::formatcp;
use log::{debug, error, info, warn};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const STORAGE_API_URL_ROOT: &str =
    "https://document-storage-production-dot-remarkable-production.appspot.com";
const AUTH_API_URL_ROOT: &str = "https://webapp-prod.cloud.remarkable.engineering";

const STORAGE_API_VERSION: &str = "1";
const STORAGE_API_URL: &str =
    formatcp!("{STORAGE_API_URL_ROOT}/service/json/{STORAGE_API_VERSION}/document-storage");

const AUTH_API_VERSION: &str = "2";
const NEW_CLIENT_URL: &str =
    formatcp!("{AUTH_API_URL_ROOT}/token/json/{AUTH_API_VERSION}/device/new");
const NEW_TOKEN_URL: &str = formatcp!("{AUTH_API_URL_ROOT}/token/json/{AUTH_API_VERSION}/user/new");

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ClientRegistation {
    code: String,
    deviceDesc: String,
    deviceID: String,
}

/// Registers a new client with the reMarkable cloud service.
///
/// This function takes a registration code and sends a request to the reMarkable
/// authentication API to register a new client device. It generates a new UUID
/// for the device ID.
///
/// # Arguments
///
/// * `code` - A string that holds the registration code provided by reMarkable.
///
/// # Returns
///
/// * `Result<String, Error>` - Returns Ok with the response text on success,
///   or an Error if the registration process fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP request fails
/// * The server responds with an error status
/// * The response cannot be parsed

pub async fn register_client(code: &str) -> Result<String, Error> {
    info!("Registering client with code: {}", code);
    let registration_info = ClientRegistation {
        code: code.to_string(),
        deviceDesc: "desktop-windows".to_string(),
        deviceID: Uuid::new_v4().to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(NEW_CLIENT_URL)
        .header("Content-Type", "application/json")
        .json(&registration_info)
        .send()
        .await?;

    log::debug!("{:?}", response);

    return match response.error_for_status() {
        Ok(res) => {
            let token = res.text().await?;
            log::debug!("Token: {}", token);
            Ok(token)
        }
        Err(e) => {
            log::error!("Error registering client: {}", e);
            Err(Error::from(e))
        }
    };
}

/// Refreshes the authentication token for the reMarkable cloud service.
///
/// This function takes an existing authentication token and sends a request to
/// the reMarkable authentication API to obtain a new, refreshed token.
///
/// # Arguments
///
/// * `auth_token` - A string that holds the current authentication token.
///
/// # Returns
///
/// * `Result<String, Error>` - Returns Ok with the new token as a string on success,
///   or an Error if the refresh process fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP request fails
/// * The server responds with an error status
/// * The response cannot be parsed
pub async fn refresh_token(auth_token: &str) -> Result<String, Error> {
    info!("Refreshing token");
    let client = reqwest::Client::new();
    let response = client
        .post(NEW_TOKEN_URL)
        .bearer_auth(auth_token)
        .header("Accept", "application/json")
        .header("Content-Length", "0")
        .send()
        .await?;

    let token = response.text().await?;
    return Ok(token);
}
