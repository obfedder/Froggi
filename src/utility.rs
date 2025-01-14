// Froggi utility library

use anyhow::{anyhow, Result};
use axum::response::IntoResponse;
use base64::prelude::*;
use rand::{thread_rng, Rng};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, read_dir};

use crate::appstate::global::*;

pub mod login {
    use anyhow::Result;
    use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
    use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use serde::{Deserialize, Serialize};
    use std::time::UNIX_EPOCH;
    use tokio::{
        fs::File,
        io::{AsyncReadExt, BufReader},
    };
    use uuid::Uuid;

    use super::Config;

    pub const API_KEY_LEN: usize = 32;

    #[derive(Serialize, Deserialize)]
    pub struct CreateLogin {
        pub username: String,
        pub password: String,
        pub confirm_password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Login {
        pub username: String,
        pub password: String,
        pub api_key: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Claims {
        sub: String,
        un: String,
        exp: usize,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SessionClaims {
        sub: String,
        exp: usize,
    }

    pub async fn auth_cookie_builder(username: String) -> Result<String> {
        if let Ok(secret) = tokio::fs::read_to_string("./secret.key").await {
            let claims = Claims {
                sub: Uuid::new_v4().to_string(),
                un: username,
                exp: (std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
                    + std::time::Duration::from_secs(60 * 60 * 24 * 7).as_secs())
                    as usize,
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )?;

            let mut config_str = String::new();

            let config_f = File::open("config.json").await?;
            let mut buf_reader = BufReader::new(config_f);
            buf_reader.read_to_string(&mut config_str).await?;

            let config: Config = serde_json::from_str(&config_str)?;

            let cookie = Cookie::build(("AuthToken", token))
                .path("/")
                .secure(config.secure_auth_cookie)
                .http_only(true)
                .max_age(cookie::time::Duration::days(7))
                .same_site(SameSite::Strict);

            Ok(cookie.to_string())
        } else {
            panic!("Failed to read secret.key!");
        }
    }

    pub async fn session_cookie_builder() -> Result<String> {
        if let Ok(secret) = tokio::fs::read_to_string("./secret.key").await {
            let claims = SessionClaims {
                sub: Uuid::new_v4().to_string(),
                exp: (std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
                    + std::time::Duration::from_secs(60 * 60 * 24 * 2).as_secs())
                    as usize,
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )?;

            let mut config_str = String::new();

            let config_f = File::open("config.json").await?;
            let mut buf_reader = BufReader::new(config_f);
            buf_reader.read_to_string(&mut config_str).await?;

            let config: Config = serde_json::from_str(&config_str)?;

            let cookie = Cookie::build(("SessionToken", token))
                .path("/")
                .secure(config.secure_auth_cookie)
                .http_only(true)
                .same_site(SameSite::Strict);

            Ok(cookie.to_string())
        } else {
            panic!("Could not read secret.key!");
        }
    }

    pub async fn verify_session(jar: CookieJar) -> Result<bool> {
        if let Some(auth_token) = jar.get("SessionToken") {
            let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

            let mut secret = String::new();

            let secret_f = File::open("secret.key").await?;
            let mut buf_reader = BufReader::new(secret_f);
            buf_reader.read_to_string(&mut secret).await?;

            if let Ok(_) = decode::<SessionClaims>(
                &auth_token.value(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            ) {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }

    pub async fn verify_auth(jar: CookieJar) -> Result<bool> {
        if let Some(auth_token) = jar.get("AuthToken") {
            let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

            let mut secret = String::new();

            let secret_f = File::open("secret.key").await?;
            let mut buf_reader = BufReader::new(secret_f);
            buf_reader.read_to_string(&mut secret).await?;

            if let Ok(_) = decode::<Claims>(
                &auth_token.value(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            ) {
                return Ok(true);
            } else {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }

    pub fn key_create(l: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(l)
            .map(char::from)
            .collect()
    }
}

pub mod hex {
    pub fn hex_to_rgb(hex: &String) -> (u8, u8, u8) {
        let hex_chars: Vec<char> = hex.trim_start_matches("#").to_string().chars().collect();

        let r = hex_char_to_u8(hex_chars[0]) * 16 + hex_char_to_u8(hex_chars[1]);
        let g = hex_char_to_u8(hex_chars[2]) * 16 + hex_char_to_u8(hex_chars[3]);
        let b = hex_char_to_u8(hex_chars[4]) * 16 + hex_char_to_u8(hex_chars[5]);

        (r, g, b)
    }

    pub fn hex_char_to_u8(c: char) -> u8 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => 15,
        }
    }

    pub fn rgb_to_hex(rgb: &(u8, u8, u8)) -> String {
        format!(
            "#{}{}{}{}{}{}",
            u8_to_hex_char((rgb.0 - (rgb.0 % 16)) / 16),
            u8_to_hex_char(rgb.0 % 16),
            u8_to_hex_char((rgb.1 - (rgb.1 % 16)) / 16),
            u8_to_hex_char(rgb.1 % 16),
            u8_to_hex_char((rgb.2 - (rgb.2 % 16)) / 16),
            u8_to_hex_char(rgb.2 % 16)
        )
    }

    pub fn u8_to_hex_char(u: u8) -> char {
        match u {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => 'F',
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub secure_auth_cookie: bool,
    pub sponsor_wait_time: u64,
    pub countdown_opacity: f32,
    pub popup_opacity: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Teaminfo {
    pub home_name: String,
    pub home_color: String,
    pub away_name: String,
    pub away_color: String,
}

impl Teaminfo {
    pub fn new() -> Teaminfo {
        Teaminfo {
            home_name: String::new(),
            home_color: String::new(),
            away_name: String::new(),
            away_color: String::new(),
        }
    }
}

pub async fn load_sponsors() -> Result<()> {
    create_dir_all(format!("./sponsors")).await?;

    let mut d = read_dir("./sponsors").await?;

    while let Ok(Some(f)) = d.next_entry().await {
        let fname = f.file_name().to_string_lossy().to_string();

        let mime_type = match fname.split(".").collect::<Vec<&str>>()[1] {
            "png" => "png",
            "jpg" => "jpeg",
            "jpeg" => "jpeg",
            _ => "",
        };

        let f_bytes = tokio::fs::read(f.path()).await?;

        *SPONSOR_IDX.lock().await = 0;
        SPONSOR_TAGS.lock().await.push(format!(
            "<img class=\"ol-sponsor-img\" src=\"data:image/{};base64,{}\" alt=\"away-img\" height=\"auto\">",
            mime_type,
            BASE64_STANDARD.encode(f_bytes),
        ))
    }

    Ok(())
}

pub async fn load_config() -> Result<()> {
    let config: Config = serde_json::from_str(&tokio::fs::read_to_string("./config.json").await?)?;

    *COUNTDOWN_OPACITY.lock().await = config.countdown_opacity;
    *POPUP_OPACITY.lock().await = config.popup_opacity;
    *SPONSOR_WAIT_TIME.lock().await = config.sponsor_wait_time;

    Ok(())
}

pub fn id_create(l: u8) -> String {
    const BASE62: &'static str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";

    let mut id = String::new();
    let base62: Vec<char> = BASE62.chars().collect();

    for _ in 0..l {
        id.push(base62[thread_rng().gen_range(0..base62.len())])
    }

    id
}

pub const REMOTE_CARGO_TOML_URL: &'static str =
    "https://raw.githubusercontent.com/AllLiver/Froggi/refs/heads/main/Cargo.toml";

pub async fn update_checker() -> Result<(bool, String)> {
    let result = reqwest::get(REMOTE_CARGO_TOML_URL).await;

    if let Ok(response) = result {
        let remote_version_str_raw = response.text().await?;
        if let Some(find_result) = remote_version_str_raw
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .find(|x| x.starts_with("version = "))
        {
            let remote_version_str = find_result
                .trim_start_matches("version = \"")
                .trim_end_matches("\"");

            let local_version_str = env!("CARGO_PKG_VERSION");

            let remote_version: Vec<u8> = remote_version_str
                .split(".")
                .map(|x| x.parse::<u8>().expect("Failed to parse remote version"))
                .collect();

            let local_version: Vec<u8> = local_version_str
                .split(".")
                .map(|x| x.parse::<u8>().expect("Failed to parse remote version"))
                .collect();

            let mut out_of_date = false;

            for i in 0..local_version.len() {
                if remote_version[i] > local_version[i] {
                    out_of_date = true;
                    break;
                } else if remote_version[i] < local_version[i] {
                    break;
                }
            }

            return Ok((out_of_date, String::from(remote_version_str)));
        } else {
            return Err(anyhow!("Failed to find version in remote cargo.toml"));
        }
    } else if let Err(e) = result {
        return Err(anyhow!("{}", e));
    } else {
        return Err(anyhow!("Some unexpected error"));
    }
}

pub struct InternalError(pub anyhow::Error);

impl IntoResponse for InternalError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for InternalError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
