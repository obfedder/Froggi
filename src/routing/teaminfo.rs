// Froggi routing (teaminfo)

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use base64::prelude::*;
use flate2::{bufread::GzDecoder, Compression, GzBuilder};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use std::{
    io::{Read, Write},
    path::PathBuf,
};
use tar::Archive;
use tokio::{
    fs::{create_dir_all, read_dir, remove_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    task::spawn_blocking,
};
use tokio_util::io::ReaderStream;

use crate::{appstate::global::*, InternalError};
use crate::{hex_to_rgb, id_create, printlg, rgb_to_hex, utility::Teaminfo, AppState};

pub async fn teaminfo_preset_create_handler(
    mut form: Multipart,
) -> Result<impl IntoResponse, InternalError> {
    let mut teaminfo = Teaminfo::new();
    let id = id_create(12);

    create_dir_all(format!("team-presets/{}", id)).await?;

    while let Some(field) = form.next_field().await? {
        if let Some(field_name) = field.name() {
            match field_name {
                "home_name" => {
                    teaminfo.home_name = field.text().await?;
                }
                "home_img" => {
                    if let Some(file_name) = field.file_name() {
                        let mut f = File::create(format!(
                            "team-presets/{}/home.{}",
                            id,
                            file_name.to_string().split(".").collect::<Vec<&str>>()[1]
                        ))
                        .await?;

                        f.write_all(field.bytes().await?.as_ref()).await?;
                    } else {
                        return Ok((
                            StatusCode::BAD_REQUEST,
                            "Failed to extract file name from form",
                        )
                            .into_response());
                    }
                }
                "home_color" => {
                    teaminfo.home_color = field.text().await?;
                }
                "away_name" => {
                    teaminfo.away_name = field.text().await?;
                }
                "away_img" => {
                    if let Some(file_name) = field.file_name() {
                        let mut f = File::create(format!(
                            "team-presets/{}/away.{}",
                            id,
                            file_name.split(".").collect::<Vec<&str>>()[1]
                        ))
                        .await?;

                        f.write_all(field.bytes().await?.as_ref()).await?;
                    } else {
                        return Ok((
                            StatusCode::BAD_REQUEST,
                            "Failed to extract file name from form",
                        )
                            .into_response());
                    }
                }
                "away_color" => {
                    teaminfo.away_color = field.text().await?;
                }
                _ => {}
            }
        } else {
            return Ok((
                StatusCode::BAD_REQUEST,
                "Failed to extract field name from form",
            )
                .into_response());
        }
    }

    let write_json = serde_json::to_string_pretty(&teaminfo)?;

    let mut f = File::create(format!("team-presets/{}/teams.json", id)).await?;
    f.write_all(write_json.as_bytes()).await?;

    printlg!(
        "CREATE teaminfo_preset: {} vs {} (id: {})",
        teaminfo.home_name,
        teaminfo.away_name,
        id
    );

    return Ok((StatusCode::OK, [("hx-trigger", "reload-selector")]).into_response());
}

pub async fn teaminfo_preset_selector_handler() -> Result<impl IntoResponse, InternalError> {
    let mut html = String::new();
    let mut a = read_dir("./team-presets").await?;

    while let Ok(Some(d)) = a.next_entry().await {
        if d.file_type().await?.is_dir() {
            let mut home_img_path = PathBuf::new();
            let mut away_img_path = PathBuf::new();

            let mut home_tag_type = "";
            let mut away_tag_type = "";

            let mut teaminfo = Teaminfo::new();

            let mut b = read_dir(d.path()).await?;

            while let Ok(Some(d0)) = b.next_entry().await {
                let file_name = d0.file_name().to_string_lossy().to_string();

                if file_name.starts_with("home.") {
                    home_img_path = d0.path();

                    home_tag_type = match file_name.split(".").collect::<Vec<&str>>()[1] {
                        "png" => "png",
                        "jpg" => "jpeg",
                        "jpeg" => "jpeg",
                        _ => "",
                    }
                } else if file_name.starts_with("away.") {
                    away_img_path = d0.path();

                    away_tag_type = match file_name.split(".").collect::<Vec<&str>>()[1] {
                        "png" => "png",
                        "jpg" => "jpeg",
                        "jpeg" => "jpeg",
                        _ => "",
                    }
                } else if file_name == "teams.json" {
                    let f = File::open(d0.path()).await?;
                    let mut buf_reader = BufReader::new(f);

                    let mut temp_str = String::new();

                    buf_reader.read_to_string(&mut temp_str).await?;

                    teaminfo = serde_json::from_str(&temp_str)?;
                }
            }
            let home_img_bytes = tokio::fs::read(home_img_path).await?;
            let away_img_bytes = tokio::fs::read(away_img_path).await?;

            let id = d.file_name().to_string_lossy().to_string();

            html += &format!(
            "<div class=\"ti-match-selector\">
                <img class=\"home-logo\" src=\"data:image/{};base64,{}\" alt=\"home-img\" height=\"30px\" width=\"auto\" style=\"border-color: {}; border-style: solid; border-radius: 3px; border-width: 2px\">
                <p class=\"ti-teampreset-title\">{} vs {}</p>
                <img class=\"away-logo\" src=\"data:image/{};base64,{}\" alt=\"away-img\" height=\"30px\" width=\"auto\" style=\"border-color: {}; border-style: solid; border-radius: 3px; border-width: 2px;\">
                <div class=\"ti-button-container\">
                    <button class=\"ti-select-button\" hx-post=\"/teaminfo/select/{}\" hx-swap=\"none\">Select</button>
                    <button class=\"ti-remove-button\" hx-post=\"/teaminfo/remove/{}\" hx-swap=\"none\">Remove</button>
                    <a class=\"ti-download-button\" href=\"/teaminfo/download-preset/{}\">Download</a>
                </div>
            </div>",
                home_tag_type,
                BASE64_STANDARD.encode(home_img_bytes),
                teaminfo.home_color,
                teaminfo.home_name,
                teaminfo.away_name,
                away_tag_type,
                BASE64_STANDARD.encode(away_img_bytes),
                teaminfo.away_color,
                id,
                id,
                id
            );
        }
    }

    return Ok(Html::from(html));
}

pub async fn teaminfo_preset_select_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, InternalError> {
    let mut dir = read_dir("./team-presets").await?;

    while let Ok(Some(a)) = dir.next_entry().await {
        if a.file_type().await?.is_dir() {
            if a.file_name().to_string_lossy().to_string() == id {
                *state.preset_id.lock().await = id.clone();

                let mut a_json = String::new();

                let a_json_f = File::open(format!(
                    "{}/teams.json",
                    a.path().to_string_lossy().to_string()
                ))
                .await?;
                let mut buf_reader = BufReader::new(a_json_f);

                buf_reader.read_to_string(&mut a_json).await?;

                let team_info: Teaminfo = serde_json::from_str(&a_json)?;

                printlg!(
                    "SELECT teaminfo_preset: {} vs {} (id: {})",
                    team_info.home_name,
                    team_info.away_name,
                    id
                );

                *state.home_name.lock().await = team_info.home_name;
                *state.away_name.lock().await = team_info.away_name;

                break;
            }
        }
    }

    return Ok(StatusCode::OK);
}

pub async fn teaminfo_preset_remove_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Ok(_) = remove_dir_all(format!("./team-presets/{}", id)).await {
        *state.preset_id.lock().await = String::new();
        printlg!("REMOVE teaminfo_preset: {}", id);
    }

    return (StatusCode::OK, [("hx-trigger", "reload-selector")]);
}

pub async fn team_name_display_handler(
    Path(t): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if t == "home" {
        return Html::from(state.home_name.lock().await.clone());
    } else if t == "away" {
        return Html::from(state.away_name.lock().await.clone());
    } else {
        return Html::from(String::new());
    }
}

pub async fn teaminfo_button_css_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, InternalError> {
    let preset_id = state.preset_id.lock().await;
    if preset_id.is_empty() {
        return Ok(StatusCode::OK.into_response());
    } else {
        if let Ok(teaminfo) = serde_json::from_str::<Teaminfo>(
            &tokio::fs::read_to_string(format!("./team-presets/{}/teams.json", *preset_id)).await?,
        ) {
            let home_rgb = hex_to_rgb(&teaminfo.home_color);
            let home_text_color =
                rgb_to_hex(&(255 - home_rgb.0, 255 - home_rgb.1, 255 - home_rgb.2));

            let away_rgb = hex_to_rgb(&teaminfo.away_color);
            let away_text_color =
                rgb_to_hex(&(255 - away_rgb.0, 255 - away_rgb.1, 255 - away_rgb.2));

            return Ok(Html::from(format!(
                "
            <style>
                .button-decrement-home {{
                    background-color: {};
                    color: {};
                }}
                .button-increment-home {{
                    background-color: {};
                    color: {};
                }}
                .button-preset-score-home {{
                    background-color: {};
                    color: {};
                }}
                .button-decrement-away {{
                    background-color: {};
                    color: {};
                }}
                .button-increment-away {{
                    background-color: {};
                    color: {};
                }}
                .button-preset-score-away {{
                    background-color: {};
                    color: {};
                }}
                .trigger-home-popup {{
                    background-color: {};
                    color: {};
                }}
                .trigger-away-popup {{
                    background-color: {};
                    color: {};
                }}
            </style>
            ",
                teaminfo.home_color,
                home_text_color,
                teaminfo.home_color,
                home_text_color,
                teaminfo.home_color,
                home_text_color,
                teaminfo.away_color,
                away_text_color,
                teaminfo.away_color,
                away_text_color,
                teaminfo.away_color,
                away_text_color,
                teaminfo.home_color,
                home_text_color,
                teaminfo.away_color,
                away_text_color,
            ))
            .into_response());
        } else {
            return Ok(StatusCode::OK.into_response());
        }
    }
}

pub async fn teaminfo_download_preset_handler(
    Path(a): Path<String>,
) -> Result<impl IntoResponse, InternalError> {
    let ti: Teaminfo = serde_json::from_str(
        &tokio::fs::read_to_string(&format!("./team-presets/{}/teams.json", a)).await?,
    )?;

    let teaminfo: Teaminfo = ti.clone();
    let id = a.clone();

    let handle: anyhow::Result<()> = spawn_blocking(move || {
        let tar_archive_path = format!("./tmp/{}.tar", id);

        let tar_archive = std::fs::File::create(&tar_archive_path)?;
        let mut builder = tar::Builder::new(tar_archive);

        let mut dir = std::fs::read_dir(format!("./team-presets/{}", id))?;

        while let Some(Ok(f)) = dir.next() {
            let mut file: std::fs::File = std::fs::File::open(f.path())?;

            builder.append_file(
                format!("./{}", f.file_name().to_string_lossy().to_string()),
                &mut file,
            )?;
        }

        builder.finish()?;

        let gz_path = format!("{}.gz", tar_archive_path);

        let gz_file = std::fs::File::create(&gz_path)?;
        let mut gz = GzBuilder::new()
            .filename(format!(
                "{}-{}.tar",
                teaminfo.home_name.clone(),
                teaminfo.away_name.clone()
            ))
            .write(gz_file, Compression::default());

        gz.write_all(&std::fs::read(tar_archive_path)?)?;
        gz.finish()?;

        Ok(())
    })
    .await?;

    handle?;

    let teaminfo_archive =
        ReaderStream::new(tokio::fs::File::open(format!("./tmp/{}.tar.gz", a)).await?);

    tokio::fs::remove_file(format!("./tmp/{}.tar.gz", a)).await?;
    tokio::fs::remove_file(format!("./tmp/{}.tar", a)).await?;

    return Ok((
        [
            (
                CONTENT_DISPOSITION,
                format!(
                    "attachment; filename=\"{}-{}.tar.gz\"",
                    ti.home_name, ti.away_name
                ),
            ),
            (CONTENT_TYPE, String::from("application/octet-stream")),
        ],
        Body::from_stream(teaminfo_archive),
    ));
}

pub async fn teaminfo_import_preset_handler(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, InternalError> {
    while let Some(f) = multipart.next_field().await? {
        if let Some(field_name) = f.name() {
            if field_name == "file" {
                let id = id_create(12);
                printlg!("IMPORT preset: {}", id);
                let gz_bytes = f.bytes().await?;

                spawn_blocking(move || {
                    let gz = GzDecoder::new(gz_bytes.as_ref());

                    let mut tar = Archive::new(gz);
                    std::fs::create_dir_all(format!("./team-presets/{}", id))?;

                    for file in tar.entries()? {
                        let mut file = file?;
                        if let Some(file_name) = file.path()?.file_name() {
                            let mut file_write = std::fs::File::create(format!(
                                "./team-presets/{}/{}",
                                id,
                                file_name.to_string_lossy().to_string()
                            ))?;

                            let mut write_buf = Vec::new();
                            file.read_to_end(&mut write_buf)?;

                            file_write.write_all(write_buf.as_ref())?;
                        } else {
                            return Err(anyhow!("Failed to get file path"));
                        }
                    }

                    return Ok(());
                })
                .await??;
            }
        }
    }

    return Ok((StatusCode::OK, [("hx-trigger", "reload-selector")]));
}
