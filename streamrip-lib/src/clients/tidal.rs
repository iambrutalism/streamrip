use base64;
use custom_error::custom_error;
use log::info;
use md5;
use regex::Regex;
use reqwest::{self, StatusCode};
use serde_json;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::{MediaType, StreamripClient};
use crate::constants;
use crate::utils::string_pair_lookup;
use crate::{GenericResult, JsonMap};

custom_error! {pub TidalError
    DeviceAuth{} = "Unable to authorize device",
    Login{mess: String} = "Login failed. Message: {mess}",
}

pub struct TidalClient {
    access_token: String,
    refresh_token: String,
    country_code: String,
    token_expiry: f64,
    web_client: reqwest::Client,
}

enum AuthStatus<'a> {
    Success(&'a [String]),
    Pending,
    Failed,
}

impl TidalClient {
    pub async fn new_user(device_code: &str) -> Result<Self, TidalError> {
        let client = reqwest::Client::new();
        let info = Self::poll_login(client)?;

        Ok(Self {
            access_token: info[0],
            refresh_token: info[1],
            country_code: info[2],
            token_expiry: info[3].parse().unwrap(),
            web_client: client,
        })
    }
    async fn poll_login(
        &client: reqwest::Client,
        device_code: &str,
    ) -> Result<AuthStatus, TidalError> {
        loop {
            let status = Self::check_auth_status(&client, device_code).await;
            match status {
                AuthStatus::Success(client) => return Some(client),
                AuthStatus::Pending => {
                    sleep(Duration::from_millis(4000)).await;
                }
                AuthStatus::Failed => {
                    return Err(TidalError::Login {
                        mess: "Authorization failed.",
                    })
                }
            }
        }

        Ok(Self {})
    }

    async fn check_auth_status(client: &reqwest::Client, device_code: &str) -> AuthStatus {
        let data = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("scope", "r_usr+w_usr+w_sub"),
            ("client_id", constants::TIDAL_CLIENT_ID),
            ("device_code", device_code),
        ];

        let resp = client
            .post(format!("{}/token", constants::TIDAL_AUTH_URL))
            .basic_auth(
                constants::TIDAL_CLIENT_ID,
                Some(constants::TIDAL_CLIENT_SECRET),
            )
            .form(&data)
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r.json::<JsonMap>().await.unwrap(),
            Err(_) => return AuthStatus::Failed,
        };

        let (status, sub_status) = (resp.get("status").unwrap(), resp.get("sub_status").unwrap());

        use serde_json::Value::Number;

        let status = if let Number(status) = status {
            status.as_u64()
        } else {
            return AuthStatus::Failed;
        };
        let sub_status = if let Number(sub_status) = sub_status {
            sub_status.as_u64()
        } else {
            return AuthStatus::Failed;
        };

        match status.unwrap() {
            200 => AuthStatus::Success(Self {}),
            400 => {
                if sub_status.unwrap() == 1002 {
                    AuthStatus::Pending
                } else {
                    AuthStatus::Failed
                }
            }
            _ => AuthStatus::Failed,
        }
    }

    pub async fn get_login_url() -> GenericResult<(String, String)> {
        let data = [
            ("client_id", constants::TIDAL_CLIENT_ID),
            ("scope", "r_usr+w_usr+w_sub"),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(format!(
                "{}/device_authorization",
                constants::TIDAL_AUTH_URL
            ))
            .form(&data)
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            return Err(Box::new(TidalError::DeviceAuth {}));
        }

        let resp = resp.json::<JsonMap>().await?;
        info!("{:?}", resp);

        let code = if let serde_json::Value::String(code) = resp.get("deviceCode").unwrap() {
            code
        } else {
            return Err(Box::new(TidalError::DeviceAuth {}));
        };
        let verification_uri =
            if let serde_json::Value::String(uri) = resp.get("verificationUri").unwrap() {
                uri
            } else {
                return Err(Box::new(TidalError::DeviceAuth {}));
            };

        Ok((code.to_string(), verification_uri.to_string()))
    }
}

impl StreamripClient for TidalClient {}

fn abs_url(epoint: &str) -> String {
    return String::from(constants::TIDAL_BASE) + epoint;
}
