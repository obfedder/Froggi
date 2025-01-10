// Froggi routing (login)

use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    http::{header::SET_COOKIE, HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    Form,
};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
};

use crate::{utility::login::*, CreateLogin, InternalError, Login};

pub async fn create_login_page_handler() -> impl IntoResponse {
    if let Ok(_) = File::open("login.json").await {
        return (StatusCode::SEE_OTHER, [("location", "/login")]).into_response();
    } else {
        return Html::from(include_str!("../html/create_login.html")).into_response();
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn create_login_handler(
    Form(data): Form<CreateLogin>,
) -> Result<impl IntoResponse, InternalError> {
    if let Ok(_) = File::open("login.json").await {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    } else if !data.username.contains(" ")
        && !data.username.is_empty()
        && !data.password.contains(" ")
        && !data.password.is_empty()
    {
        if data.password == data.confirm_password {
            let password_hash: Result<String, anyhow::Error> =
                tokio::task::spawn_blocking(move || {
                    let salt = SaltString::generate(&mut OsRng);
                    let argon2 = Argon2::default();
                    if let Ok(hash) = argon2.hash_password(data.password.as_bytes(), &salt) {
                        return Ok(hash.to_string());
                    } else {
                        return Err(anyhow!("Failed to hash password"));
                    }
                })
                .await?;

            let mut f = File::create("login.json").await?;

            let new_login = Login {
                username: data.username,
                password: password_hash?,
                api_key: key_create(API_KEY_LEN),
            };

            f.write_all(serde_json::to_string(&new_login)?.as_bytes())
                .await?;

            return Ok((
                StatusCode::SEE_OTHER,
                [
                    ("set-cookie", auth_cookie_builder(new_login.username).await?),
                    ("hx-redirect", String::from("/")),
                ],
            )
                .into_response());
        } else {
            return Ok(
                "<div class=\"login-error\"><p>Passwords do not match</p></div>".into_response(),
            );
        }
    } else {
        return Ok("<div class=\"login-error\"><p>Username and password cannot be empty or contain spaces</p></div>".into_response());
    }
}

pub async fn login_page_handler() -> impl IntoResponse {
    if let Ok(_) = File::open("login.json").await {
        return Html::from(include_str!("../html/login.html")).into_response();
    } else {
        return (StatusCode::SEE_OTHER, [("location", "/login/create")]).into_response();
    }
}

pub async fn login_handler(
    Form(data): Form<LoginForm>,
) -> Result<impl IntoResponse, InternalError> {
    if let Ok(f) = File::open("login.json").await {
        if !data.username.contains(" ")
            && !data.username.is_empty()
            && !data.password.contains(" ")
            && !data.password.is_empty()
        {
            let mut buf = String::new();
            let mut buf_reader = BufReader::new(f);

            buf_reader.read_to_string(&mut buf).await?;

            let hashed_login: Login = serde_json::from_str(&buf)?;

            if data.username == hashed_login.username {
                let handle: tokio::task::JoinHandle<Result<bool, anyhow::Error>> =
                    tokio::task::spawn_blocking(move || {
                        if let Ok(h) = PasswordHash::new(&hashed_login.password) {
                            return Ok(Argon2::default()
                                .verify_password(data.password.as_bytes(), &h)
                                .is_ok());
                        } else {
                            return Err(anyhow::anyhow!("Failed to hash password"));
                        }
                    });

                if handle.await?? {
                    return Ok(([
                        ("set-cookie", auth_cookie_builder(data.username).await?),
                        ("hx-redirect", String::from("/")),
                    ])
                    .into_response());
                } else {
                    return Ok(
                        "<div class=\"login-error\"><p>Invalid login</p></div>".into_response()
                    );
                }
            } else {
                return Ok("<div class=\"login-error\"><p>Invalid login</p></div>".into_response());
            }
        } else {
            return Ok("<div class=\"login-error\"><p>Username and password cannot be empty or contain spaces</p></div>".into_response());
        }
    } else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }
}

pub async fn logout_handler() -> Result<impl IntoResponse, InternalError> {
    return Ok(Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, Cookie::build(("AuthToken", "0")).to_string())
        .header(SET_COOKIE, Cookie::build(("SessionToken", "0")).to_string())
        .header(
            HeaderName::from_static("hx-redirect"),
            HeaderValue::from_static("/"),
        )
        .body(String::new())?);
}
