// Froggi routing (sponsors)

use std::path::PathBuf;

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use base64::prelude::*;
use tokio::{
    fs::{create_dir_all, read_dir, remove_file, File},
    io::AsyncWriteExt,
};

use crate::{appstate::global::*, id_create, printlg, utility::load_sponsors, InternalError};

pub async fn upload_sponsors_handler(
    mut form: Multipart,
) -> Result<impl IntoResponse, InternalError> {
    create_dir_all(format!("./sponsors")).await?;

    while let Some(field) = form.next_field().await? {
        let id = id_create(12);
        if let Some(fname) = field.file_name() {
            let mut f = File::create(format!(
                "./sponsors/{}.{}",
                id,
                fname.split(".").collect::<Vec<&str>>()[1]
            ))
            .await?;

            f.write_all(field.bytes().await?.as_ref()).await?;

            printlg!("ADD sponsor: {}", id);
        }
    }

    load_sponsors().await?;

    return Ok((StatusCode::OK, [("hx-trigger", "reload-sponsor")]));
}

pub async fn sponsors_management_handler() -> Result<impl IntoResponse, InternalError> {
    let mut d = read_dir("./sponsors").await?;
    let mut html = String::new();

    while let Ok(Some(a)) = d.next_entry().await {
        let fname = a.file_name().to_string_lossy().to_string();
        let fname_vec = fname.split(".").collect::<Vec<&str>>();

        let mime = match fname_vec[1] {
            "png" => "png",
            "jpg" => "jpeg",
            "jpeg" => "jpeg",
            _ => "",
        };

        let f_bytes = tokio::fs::read(a.path()).await?;

        html += &format!(
            "<div class=\"ti-sponsor-wrapper\">
                <img src=\"data:image/{};base64,{}\" alt=\"away-img\" height=\"30px\" width=\"auto\">
                <button class=\"ti-sponsor-remove-button\" hx-post=\"/sponsors/remove/{}\" hx-swap=\"none\">Remove</button>
            </div>",
            mime,
            BASE64_STANDARD.encode(f_bytes),
            fname_vec[0]
        );
    }

    return Ok(Html::from(html));
}

pub async fn sponsor_remove_handler(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, InternalError> {
    let mut d = read_dir("./sponsors").await?;
    let mut p = PathBuf::new();

    while let Ok(Some(a)) = d.next_entry().await {
        if a.file_name()
            .to_string_lossy()
            .to_string()
            .split(".")
            .collect::<Vec<&str>>()[0]
            == id
        {
            p = a.path();
            break;
        }
    }

    remove_file(p).await?;

    printlg!("REMOVE sponsor: {}", id);

    return Ok((StatusCode::OK, [("hx-trigger", "reload-sponsor")]));
}
