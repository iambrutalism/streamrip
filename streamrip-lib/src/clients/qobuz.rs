use std::iter::Iterator;
use std::str;
use std::time::SystemTime;

// use hashbrown::HashMap;
use base64;
use custom_error::custom_error;
use log::info;
use md5;
use regex::Regex;
use reqwest::{self, StatusCode};
use serde_json;

use crate::base::{MediaType, StreamripClient};
use crate::constants;
use crate::utils::str_pair_lookup;
use crate::{GenericResult, JsonMap};

custom_error! { pub QobuzError
    Spoof{} = "Unable to find app ID and secrets.",
    Login{message: String} = "Login failed. Message: {message}",
    InvalidQuality{q: u32} = "Invalid quality {q}. Must be integer from 0 to 4.",
    StreamError {message: String} = "Unable to stream item. Message: {message}",
    InvalidSecret {sec: String} = "Invalid secret '{sec}'. Run [RESET COMMAND]",
}

pub async fn get_app_id_and_secrets() -> GenericResult<(String, String)> {
    let client = reqwest::Client::new();

    let resp = client.get("https://play.qobuz.com/login").send().await?;

    let re_bundle_url =
        Regex::new(r#"<script src="(/resources/\d+\.\d+\.\d+-[a-z]\d{3}/bundle\.js)"></script>"#)
            .unwrap();

    let mut bundle_url: Option<String> = None;
    let text = resp.text_with_charset("utf-8").await?;
    let possible_match = re_bundle_url.captures(&text);
    if let Some(url_match) = possible_match {
        if let Some(url) = url_match.get(1) {
            bundle_url = Some(url.as_str().to_string());
        }
    }

    let bundle_url_option = bundle_url;
    let bundle_url: String;
    if let Some(url) = bundle_url_option {
        bundle_url = url;
    } else {
        return Err(Box::new(QobuzError::Spoof {}));
    }

    let main_page_text = client
        .get("https://play.qobuz.com".to_owned() + &bundle_url)
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;

    let re_seed_timezone =
        Regex::new(r#"[a-z]\.initialSeed\("([\w=]+)",window\.utimezone\.([a-z]+)\)"#).unwrap();

    let mut timezone_seed: Vec<(&str, &str)> = re_seed_timezone
        .captures_iter(&main_page_text)
        .map(|x| (x.get(2).unwrap().as_str(), x.get(1).unwrap().as_str()))
        .collect();

    info!("timezone seed pairs {:?}", timezone_seed);

    let extras_regex = format!(
        r#"name:"\w+/({timezones})",info:"([\w=]+)",extras:"([\w=]+)""#,
        timezones = timezone_seed
            .iter_mut()
            .map(|(tz, _)| { capitalize(tz.to_string()) })
            .collect::<Vec<_>>()
            .join("|")
    );

    let re_extras = Regex::new(&extras_regex).unwrap();

    let info_extras: Vec<(&str, &str, &str)> = re_extras
        .captures_iter(&main_page_text)
        .map(|x| {
            (
                x.get(1).unwrap().as_str(),
                x.get(2).unwrap().as_str(),
                x.get(3).unwrap().as_str(),
            )
        })
        .collect();

    let mut secrets: Vec<String> = vec![];
    for (tz1, info, extras) in info_extras {
        let mut tz1: String = tz1.to_string(); // remove this later
        tz1.make_ascii_lowercase();
        let seed = str_pair_lookup(&timezone_seed, &tz1).unwrap().to_string();
        // let seed = timezone_seed
        //     .iter()
        //     .find_map(|(tz2, seed)| if *tz2 == tz1 { Some(*seed) } else { None })
        //     .unwrap()
        //     .to_string();

        let key = seed + info + extras;
        let key = &key[0..(key.len() - 44)];
        let secret = str::from_utf8(&base64::decode(key).unwrap())
            .unwrap()
            .to_string();
        secrets.push(secret);
    }

    info!("Secrets found: {:?}", secrets);
    // let mut secrets = timezone_seed
    //     .iter()
    //     .zip(info_extras)
    //     .map(|((_, s), (i, e))| {
    //         let key = s.to_string() + i + e;
    //         let key = &key[0..(key.len() - 44)];
    //         str::from_utf8(&base64::decode(key).unwrap())
    //             .unwrap()
    //             .to_string()
    //     })
    //     .collect::<Vec<_>>();

    secrets.swap(0, 1);
    let secrets: String = secrets.join(";");

    let re_app_id = Regex::new(
        r#"\{app_id:"(\d{9})",app_secret:"\w{32}",base_port:"80",base_url:"https://www\.qobuz\.com",base_method:"/api\.json/0\.2/"\},n\.base_url="https://play\.qobuz\.com""#,
    ).unwrap();

    let app_id: String = re_app_id
        .captures(&main_page_text)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    // let last_secret = matches.remove(1);

    // let secrets: Vec<String> = vec![];
    // let seed = matches.get(1).unwrap().as_str().to_string();
    // let timezone = matches.get(1).unwrap().as_str().to_string();

    // let info_extras_unformatted = ;
    // let info_extras_formatted =
    // let re_info_extras = Regex::new().unwrap();

    Ok((app_id, secrets))
}

#[derive(Debug)]
pub struct QobuzClient {
    // None if not logged in
    // Should be stored in client?
    app_id: String,
    secrets: String,
    web_client: reqwest::Client,
    auth_info: Vec<(String, String)>,
}

impl QobuzClient {
    pub async fn new(auth_info: Vec<(String, String)>) -> GenericResult<Self> {
        let (app_id, secrets) = get_app_id_and_secrets().await?;
        info!("app_id: {}", app_id);
        info!("secrets: {}", app_id);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-App-Id", app_id.parse().unwrap());

        let web_client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(constants::AGENT)
            .build()?;

        #[cfg(debug_assertions)]
        assert!(auth_info.len() == 2);

        Ok(Self {
            auth_info,
            app_id,
            secrets,
            web_client,
        })
    }

    pub async fn with_tokens(
        auth_info: Vec<(String, String)>,
        app_id: String,
        secrets: String,
    ) -> GenericResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-App-Id", app_id.parse().unwrap());

        let web_client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(constants::AGENT)
            .build()?;

        Ok(Self {
            auth_info,
            app_id,
            secrets,
            web_client,
        })
    }

    pub async fn login(&mut self) -> GenericResult<()> {
        self.auth_info
            .push(("app_id".to_string(), self.app_id.clone()));

        let (login_resp, secret) = tokio::join!(
            self.web_client
                .get(abs_url("user/login"))
                .query(&self.auth_info)
                .send(),
            self.validate_secret()
        );

        let login_resp = login_resp?;
        self.secrets = secret?;

        match login_resp.status() {
            StatusCode::OK => (),
            // TODO: check for credential parameter for free accounts
            StatusCode::UNAUTHORIZED => {
                return Err(Box::new(QobuzError::Login {
                    message: "Invalid credentials".to_string(),
                }))
            }
            StatusCode::BAD_REQUEST => {
                return Err(Box::new(QobuzError::Login {
                    message: "Invalid app ID".to_string(),
                }))
            }
            other => {
                return Err(Box::new(QobuzError::Login {
                    message: format!("Unknown error logging in. Code: {}", other),
                }))
            }
        };

        let resp_json = login_resp.json::<JsonMap>().await?;

        let auth_token =
            if let Some(serde_json::Value::String(uat)) = resp_json.get("user_auth_token") {
                uat.parse().unwrap()
            } else {
                return Err(Box::new(QobuzError::Login {
                    message: "Could not find uat".to_string(),
                }));
            };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-App-Id", self.app_id.parse().unwrap());
        headers.insert("X-User-Auth-Token", auth_token);

        self.web_client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(constants::AGENT)
            .build()?;

        Ok(())
    }

    pub async fn get_file_url(&self, id: &str, quality: u32) -> GenericResult<String> {
        self._get_file_url(id, quality, &self.secrets).await
    }

    pub async fn get_metadata(
        &self,
        id: &str,
        media_type: MediaType,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> GenericResult<JsonMap> {
        let limit = limit.unwrap_or(500);
        let offset = offset.unwrap_or(0);

        let mut params: Vec<(String, String)> = vec![
            ("app_id".to_string(), self.app_id.clone()),
            (format!("{}_id", media_type.to_string()), id.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("offset".to_string(), offset.to_string()),
        ];

        const EXTRAS: [(&str, &str); 3] = [
            ("artist", "albums"),
            ("playlist", "tracks"),
            ("label", "albums"),
        ];

        let media_type_s = media_type.to_string();
        let extra: Option<&str> = str_pair_lookup(&EXTRAS, &media_type_s);
        // EXTRAS
        //     .iter()
        //     .find_map(|(x, y)| if *x == &media_type_s { Some(*y) } else { None });

        if let Some(extra) = extra {
            params.push((media_type_s, extra.to_string()));
        }

        let path = format!("{}/get", media_type);
        let resp = self
            .web_client
            .get(abs_url(&path))
            .query(&params)
            .send()
            .await?;

        assert!(resp.status() == StatusCode::OK);

        let json = resp.json::<JsonMap>().await?;
        info!("{:?}", json);
        Ok(json)
    }

    pub async fn search(
        &self,
        query: &str,
        media_type: MediaType,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> GenericResult<JsonMap> {
        let limit = limit.unwrap_or(50).to_string();
        let offset = offset.unwrap_or(0).to_string();

        let mut epoint = media_type.to_string() + "/search";
        if query == "<streamrip::featured>" {
            epoint = "album/getFeatured".to_string();
        }

        let epoint = abs_url(&epoint);

        let params = [("query", query), ("limit", &limit), ("offset", &offset)];

        let resp = self.web_client.get(&epoint).query(&params).send().await?;

        assert!(resp.status() == StatusCode::OK);

        let json = resp.json::<JsonMap>().await?;
        info!("search result: {:?}", json);
        Ok(json)
    }

    async fn _get_file_url(&self, id: &str, quality: u32, secret: &str) -> GenericResult<String> {
        let unix_ts: f64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        if quality < 1 || quality > 4 {
            return Err(Box::new(QobuzError::InvalidQuality { q: quality }));
        }

        // subtract 1 because 0 is not available
        let qobuz_quality = constants::QOBUZ_QUALITY_MAP[quality as usize - 1];

        let signature = format!(
            "trackgetFileUrlformat_id{}intentstreamtrack_id{}{}{}",
            qobuz_quality, id, unix_ts, secret
        );

        info!("signature_prehash: {} with secret: {}", signature, secret);

        let hashed_signature = format!("{:?}", md5::compute(&signature));

        info!("hashed signature: {}", hashed_signature);

        let params: [(String, String); 5] = [
            ("request_ts".to_string(), unix_ts.to_string()),
            ("request_sig".to_string(), hashed_signature),
            ("track_id".to_string(), id.to_string()),
            ("format_id".to_string(), qobuz_quality.to_string()),
            ("intent".to_string(), "stream".to_string()),
        ];

        let resp = self
            .web_client
            .get(abs_url("track/getFileUrl"))
            .query(&params)
            .send()
            .await?;

        match resp.status() {
            StatusCode::BAD_REQUEST => {
                return Err(Box::new(QobuzError::Login {
                    message: format!(
                        "Inavlid app id from params: {:?}, sig: {}",
                        params, signature
                    ),
                }))
            }
            _ => (),
        }

        let resp = resp.json::<JsonMap>().await.unwrap();

        info!("{}", format!("file url: {:?}", resp));
        Ok(format!("file url: {:?}", resp))
    }

    async fn validate_secret(&self) -> Result<String, QobuzError> {
        let secrets: Vec<&str> = self.secrets.split(';').collect();

        if secrets.len() == 1 {
            // prevalidated secret
            return Ok(secrets[0].to_string());
        }

        let (sec1, sec2) =
            tokio::join!(self.check_secret(secrets[0]), self.check_secret(secrets[1]));

        info!("{:?}, {:?}", sec1, sec2);
        if let Some(sec) = sec1 {
            return Ok(sec.to_string());
        }
        if let Some(sec) = sec2 {
            return Ok(sec.to_string());
        }
        return Err(QobuzError::InvalidSecret {
            sec: format!("{:?}", self.secrets),
        });
    }

    async fn check_secret<'a>(&self, secret: &'a str) -> Option<&'a str> {
        match self._get_file_url("19512574", 1, secret).await {
            Ok(_) => Some(secret),
            Err(_) => None,
        }
    }
}

impl StreamripClient for QobuzClient {}

fn capitalize(mut s: String) -> String {
    if s.len() > 0 {
        unsafe {
            let v = s.as_mut_vec();
            v[0] -= 32;
        }
    }
    s
}

fn abs_url(epoint: &str) -> String {
    return String::from(constants::QOBUZ_BASE) + epoint;
}
