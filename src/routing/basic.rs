// Froggi routing (basic)

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use reqwest::header::CONTENT_TYPE;
use std::{
    io::{Cursor, Read},
    path::PathBuf,
};
use tar::Archive;
use tokio::{fs::File, task::spawn_blocking};

use crate::InternalError;

pub async fn index_handler() -> impl IntoResponse {
    Html::from(include_str!("../html/index.html"))
}

pub async fn teaminfo_handler() -> impl IntoResponse {
    Html::from(include_str!("../html/teaminfo.html"))
}

pub async fn overlay_handler() -> impl IntoResponse {
    if let Ok(_) = File::open("login.json").await {
        return Html::from(include_str!("../html/overlay.html")).into_response();
    } else {
        return (StatusCode::SEE_OTHER, [("location", "/login/create")]).into_response();
    }
}

pub async fn settings_handler() -> impl IntoResponse {
    Html::from(include_str!("../html/settings.html"))
}

pub async fn logs_page_handler() -> impl IntoResponse {
    Html::from(include_str!("../html/logs.html"))
}

pub async fn css_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/css")],
        include_str!("../html/styles.css"),
    )
}

pub async fn not_found_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        include_str!("../html/status_codes/404.html"),
    )
}

pub async fn htmx_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/htmx.js"),
    )
}

pub async fn ws_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/ws.js"),
    )
}

pub async fn app_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/app.js"),
    )
}

pub async fn index_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/index.js"),
    )
}

pub async fn overlay_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/overlay.js"),
    )
}

pub async fn teaminfo_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/index.js"),
    )
}

pub async fn sanitize_js_handler() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/javascript")],
        include_str!("../html/js/sanitize.js"),
    )
}

pub async fn local_asset_handler(
    Path((dir, asset)): Path<(String, String)>,
) -> Result<Response<Body>, InternalError> {
    let handle: tokio::task::JoinHandle<Result<Response<Body>, anyhow::Error>> = spawn_blocking(
        move || {
            let tar_archive_bytes = Cursor::new(include_bytes!("../../build-tmp/local-assets.tar"));
            let desired_path = PathBuf::from(format!("{}/{}", dir, asset));

            let mut tar_archive = Archive::new(tar_archive_bytes);
            let mut entries = tar_archive.entries()?;

            while let Some(Ok(mut f)) = entries.next() {
                let f_path = f.path()?;
                let f_ext = f_path.extension();
                if f_path.to_path_buf() == desired_path {
                    if let Some(ext) = f_ext {
                        let mime_type: Option<&'static str> = match ext.to_string_lossy().to_string().as_ref() { // All mime types covered (except 3gp and 3g2, as of 1/7/25) in https://developer.mozilla.org/en-US/docs/Web/HTTP/MIME_types/Common_types
                        "aac" => Some("audio/aac"),
                        "abw" => Some("application/x-abiword"),
                        "apng" => Some("image/apng"),
                        "arc" => Some("application/x-freearc"),
                        "avif" => Some("image/avif"),
                        "avi" => Some("video/x-msvideo"),
                        "azw" => Some("application/vnd.amazon.ebook"),
                        "bin" => Some("application/octet-stream"),
                        "bmp" => Some("image/bmp"),
                        "bz" => Some("application/x-bzip"),
                        "bz2" => Some("application/x-bzip2"),
                        "cda" => Some("application/x-cdf"),
                        "csh" => Some("application/x-csh"),
                        "css" => Some("text/css"),
                        "csv" => Some("text/csv"),
                        "doc" => Some("application/msword"),
                        "docx" => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
                        "eot" => Some("application/vnd.ms-fontobject"),
                        "epub" => Some("application/epub+zip"),
                        "gz" => Some("application/gzip"),
                        "gif" => Some("image/gif"),
                        "htm" => Some("text/html"),
                        "html" => Some("text/html"),
                        "ico" => Some("image/vnd.microsoft.icon"),
                        "ics" => Some("text/calendar"),
                        "jar" => Some("application/java-archive"),
                        "jpeg" => Some("image/jpeg"),
                        "jpg" => Some("image/jpeg"),
                        "js" => Some("text/javascript"),
                        "json" => Some("application/json"),
                        "jsonld" => Some("application/ld+json"),
                        "mid" => Some("audio/midi"),
                        "midi" => Some("audio/midi"),
                        "mjs" => Some("text/javascript"),
                        "mp3" => Some("audio/mpeg"),
                        "mp4" => Some("video/mp4"),
                        "mpeg" => Some("video/mpeg"),
                        "mpkg" => Some("application/vnd.apple.installer+xml"),
                        "odp" => Some("application/vnd.oasis.opendocument.presentation"),
                        "ods" => Some("application/vnd.oasis.opendocument.spreadsheet"),
                        "odt" => Some("application/vnd.oasis.opendocument.text"),
                        "oga" => Some("audio/ogg"),
                        "ogv" => Some("video/ogg"),
                        "ogx" => Some("application/ogg"),
                        "opus" => Some("audio/ogg"),
                        "otf" => Some("font/otf"),
                        "png" => Some("image/png"),
                        "pdf" => Some("application/pdf"),
                        "php" => Some("application/x-httpd-php"),
                        "ppt" => Some("application/vnd.ms-powerpoint"),
                        "pptx" => Some("application/vnd.openxmlformats-officedocument.presentationml.presentation"),
                        "rar" => Some("application/vnd.rar"),
                        "rtf" => Some("application/rtf"),
                        "sh" => Some("application/x-sh"),
                        "svg" => Some("image/svg+xml"),
                        "tar" => Some("application/x-tar"),
                        "tif" => Some("image/tiff"),
                        "tiff" => Some("image/tiff"),
                        "ts" => Some("video/mp2t"),
                        "ttf" => Some("font/ttf"),
                        "txt" => Some("text/plain"),
                        "vsd" => Some("application/vnd.visio"),
                        "wav" => Some("audio/wav"),
                        "weba" => Some("audio/webm"),
                        "webm" => Some("video/webm"),
                        "webp" => Some("image/webp"),
                        "woff" => Some("font/woff"),
                        "woff2" => Some("font/woff2"),
                        "xhtml" => Some("application/xhtml+xml"),
                        "xls" => Some("application/vnd.ms-excel"),
                        "xlsx" => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
                        "xml" => Some("application/xml"),
                        "xul" => Some("application/vnd.mozilla.xul+xml"),
                        "zip" => Some("application/zip"),
                        // 3gp and 3g2 excluded
                        "7z" => Some("application/x-7z-compressed"),
                        _ => None
                    };

                        if let Some(m) = mime_type {
                            let mut bytes_vec = Vec::new();
                            f.read_to_end(&mut bytes_vec)?;

                            return Ok(Response::builder()
                                .status(StatusCode::OK)
                                .header(CONTENT_TYPE, m)
                                .body(Body::from(bytes_vec))?);
                        } else {
                            return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(
                                Body::from("Requested file without corresponding mime type"),
                            )?);
                        }
                    } else {
                        return Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Requested directory or file without extension"))?);
                    }
                }
            }
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Requested file doesn't exist"))?);
        },
    );

    return Ok(handle.await??);
}
