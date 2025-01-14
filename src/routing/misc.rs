// Froggi routing (misc)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use std::collections::HashMap;

use crate::{appstate::global::*, load_config, printlg, AppState, Config, InternalError};

pub async fn popup_handler(
    Path(a): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let Some(p) = params.get("text") {
        if a == "home" {
            POPUPS_HOME.lock().await.push((p.clone(), 7));
        } else if a == "away" {
            POPUPS_AWAY.lock().await.push((p.clone(), 7));
        }
        printlg!("POPUP: {}", p);
    }

    return StatusCode::OK;
}

pub async fn reset_handler(State(ref mut state): State<AppState>) -> impl IntoResponse {
    *state.home_points.lock().await = 0;
    *state.home_name.lock().await = String::from("Home");
    *state.away_points.lock().await = 0;
    *state.away_name.lock().await = String::from("Away");
    *state.quarter.lock().await = 1;
    *state.preset_id.lock().await = String::new();
    *state.down.lock().await = 1;
    *state.downs_togo.lock().await = 1;
    *state.countdown_text.lock().await = String::from("Countdown");
    *state.show_countdown.lock().await = false;
    *state.show_downs.lock().await = true;
    *state.show_scoreboard.lock().await = true;

    *GAME_CLOCK.lock().await = 0;
    *GAME_CLOCK_START.lock().await = false;
    *COUNTDOWN_CLOCK.lock().await = 0;
    *COUNTDOWN_CLOCK_START.lock().await = false;
    *SHOW_SPONSORS.lock().await = false;
    *OCR_API.lock().await = false;

    printlg!("SCOREBOARD REST");

    return StatusCode::OK;
}

pub async fn logs_handler() -> impl IntoResponse {
    let logs = LOGS.lock().await;
    let mut logs_display = Vec::new();

    for i in (0..logs.len()).rev() {
        logs_display.push(format!("<span>({}) {}</span>", i + 1, logs[i]))
    }

    Html::from(logs_display.join("<br>"))
}

pub async fn restart_handler() -> impl IntoResponse {
    printlg!("Restarting...");

    if let Some(tx) = RESTART_SIGNAL.lock().await.take() {
        let _ = tx.send(());
        return (StatusCode::OK, "Restarting...");
    } else {
        printlg!("Restart signal already sent");
        return (StatusCode::METHOD_NOT_ALLOWED, "Restart already sent!");
    }
}

pub async fn shutdown_handler() -> impl IntoResponse {
    printlg!("Shutting down...");

    if let Some(tx) = SHUTDOWN_SIGNAL.lock().await.take() {
        let _ = tx.send(());
        return (StatusCode::OK, "Shutting down...");
    } else {
        printlg!("Shutdown signal already sent");
        return (StatusCode::METHOD_NOT_ALLOWED, "Shutdown already sent!");
    }
}

pub async fn ping_handler() -> impl IntoResponse {
    return StatusCode::OK;
}

pub async fn config_json_form_handler() -> Result<impl IntoResponse, InternalError> {
    let cfg: Config = serde_json::from_str(&tokio::fs::read_to_string("./config.json").await?)?;

    return Ok(Html::from(format!(
        "<form hx-post=\"/config-json/set\" hx-swap=\"outerHTML\">
            <label for=\"sponsor-wait-time\">Sponsor roll time:</label>
            <input type=\"number\" name=\"sponsor-wait-time\" placeholder=\"{}\" class=\"number-input\" min=\"0\" >
            
            <label for=\"countdown-opacity\">Countdown opacity:</label>
            <input type=\"number\" name=\"countdown-opacity\" placeholder=\"{}\" class=\"number-input\" min=\"0\" max=\"1\" step=\"0.1\">
                
            <label for=\"popup-opacity\">Popup opacity:</label>
            <input type=\"number\" name=\"popup-opacity\" placeholder=\"{}\" class=\"number-input\" min=\"0\" max=\"1\" step=\"0.1\">
                
            <input type=\"submit\" value=\"Submit\" class=\"settings-submit\">
            <img class=\"htmx-indicator\" src=\"/favicon.png\"></img>
        </form>",
        cfg.sponsor_wait_time, cfg.countdown_opacity, cfg.popup_opacity
    )));
}

pub async fn config_json_set_handler(
    Form(set_cfg): Form<HashMap<String, String>>,
) -> Result<impl IntoResponse, InternalError> {
    let mut config: Config =
        serde_json::from_str(&tokio::fs::read_to_string("./config.json").await?)?;

    if let Some(val) = set_cfg.get("sponsor-wait-time") {
        if let Ok(sponsor_wait_time) = val.parse::<u64>() {
            config.sponsor_wait_time = sponsor_wait_time.clone();
        }
    }

    if let Some(val) = set_cfg.get("countdown-opacity") {
        if let Ok(countdown_opacity) = val.parse::<f32>() {
            if countdown_opacity >= 0.0 && countdown_opacity <= 1.0 {
                config.countdown_opacity = countdown_opacity.clone() as f32;
            }
        }
    }

    if let Some(val) = set_cfg.get("popup-opacity") {
        if let Ok(popup_opacity) = val.parse::<f32>() {
            if popup_opacity >= 0.0 && popup_opacity <= 1.0 {
                config.popup_opacity = popup_opacity.clone() as f32;
            }
        }
    }

    tokio::fs::write("./config.json", serde_json::to_string_pretty(&config)?).await?;
    load_config().await?;

    printlg!("SET config_json: {:?}", config);

    return Ok(Html::from(format!(
        "<form hx-post=\"/config-json/set\" hx-swap=\"outerHTML\">
        <label for=\"sponsor-wait-time\">Sponsor roll time:</label>
        <input type=\"number\" name=\"sponsor-wait-time\" placeholder=\"{}\" class=\"number-input\">
        
        <label for=\"countdown-opacity\">Countdown opacity:</label>
        <input type=\"number\" name=\"countdown-opacity\" placeholder=\"{}\" class=\"number-input\">
            
        <label for=\"popup-opacity\">Popup opacity:</label>
        <input type=\"number\" name=\"popup-opacity\" placeholder=\"{}\" class=\"number-input\">
            
        <input type=\"submit\" value=\"Submit\">
        <img class=\"htmx-indicator\" src=\"/local-asset/png/favicon.png\" class=\"settings-submit\"></img>
    </form>",
        config.sponsor_wait_time, config.countdown_opacity, config.popup_opacity
    )));
}
