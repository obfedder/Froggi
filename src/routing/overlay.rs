// Froggi routing (overlay)

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use base64::prelude::*;
use tokio::fs::read_dir;

use crate::{AppState, InternalError, Teaminfo};

pub async fn icon_handler(
    Path(t): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, InternalError> {
    let mut d = read_dir(format!("./team-presets/{}", state.preset_id.lock().await)).await?;

    while let Ok(Some(f)) = d.next_entry().await {
        if !f.file_type().await?.is_dir() {
            let fname = f.file_name().to_string_lossy().to_string();

            if t == "home" && fname.starts_with("home.") {
                let img_bytes = tokio::fs::read(f.path()).await?;

                let mime_type = match fname.split(".").collect::<Vec<&str>>()[1] {
                    "png" => "png",
                    "jpg" => "jpeg",
                    "jpeg" => "jpeg",
                    _ => "",
                };

                return Ok(Html::from(format!("<img class=\"ol-team-logo\" src=\"data:image/{};base64,{}\" height=\"30px\" width=\"auto\" alt=\"home-icon\">", mime_type, BASE64_STANDARD.encode(img_bytes))));
            } else if t == "away" && fname.starts_with("away.") {
                let img_bytes = tokio::fs::read(f.path()).await?;

                let mime_type = match fname.split(".").collect::<Vec<&str>>()[1] {
                    "png" => "png",
                    "jpg" => "jpeg",
                    "jpeg" => "jpeg",
                    _ => "",
                };

                return Ok(Html::from(format!("<img class=\"ol-team-logo\" src=\"data:image/{};base64,{}\" height=\"30px\" width=\"auto\" alt=\"away-icon\">", mime_type, BASE64_STANDARD.encode(img_bytes))));
            }
        }
    }

    return Ok(Html::from(String::new()));
}

pub async fn overlay_team_border_css_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, InternalError> {
    let preset_id = state.preset_id.lock().await;
    if preset_id.is_empty() {
        return Ok(Html::from(String::new()));
    } else {
        if let Ok(teaminfo) = serde_json::from_str::<Teaminfo>(
            &tokio::fs::read_to_string(format!("./team-presets/{}/teams.json", *preset_id)).await?,
        ) {
            return Ok(Html::from(format!(
                "
            <style>
                .ol-home-box, .button {{
                    border-color: {};
                    border-style: solid;
                }}
                .ol-away-box {{
                    border-color: {};
                    border-style: solid;
                }}
            </style>
            ",
                teaminfo.home_color, teaminfo.away_color
            )));
        } else {
            return Ok(Html::from(String::new()));
        }
    }
}
