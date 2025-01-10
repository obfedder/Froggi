// Froggi middleware

use axum::{
    extract::Request,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{utility::login::*, InternalError};

pub async fn auth_session_layer(
    jar: CookieJar,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, InternalError> {
    if verify_session(jar).await? {
        return Ok(next.run(request).await);
    } else {
        if let Some(h) = headers.get("api-auth") {
            let login: Login =
                serde_json::from_str(&tokio::fs::read_to_string("./login.json").await?)?;
            if h.to_str()? == login.api_key {
                return Ok(next.run(request).await);
            } else {
                return Ok(StatusCode::UNAUTHORIZED.into_response());
            }
        } else {
            return Ok(StatusCode::UNAUTHORIZED.into_response());
        }
    }
}

pub async fn auth_give_session_layer(
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, InternalError> {
    if verify_auth(jar.clone()).await? || verify_session(jar.clone()).await? {
        if verify_session(jar).await? {
            return Ok(next.run(request).await);
        } else {
            let mut response = next.run(request).await;
            let cookie = session_cookie_builder().await;

            response
                .headers_mut()
                .append(SET_COOKIE, HeaderValue::from_str(&cookie?)?);

            return Ok(response);
        }
    } else {
        return Ok((StatusCode::SEE_OTHER, [("location", "/login")]).into_response());
    }
}
