// Froggi routing (updating)

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{appstate::global::*, printlg, update_checker, REMOTE_CARGO_TOML_URL};

pub async fn update_handler() -> impl IntoResponse {
    printlg!("Checking if an update is availible...");

    if let Ok(result) = update_checker().await {
        if result.0 {
            if let Some(tx) = UPDATE_SIGNAL.lock().await.take() {
                printlg!("Starting update...");
                let _ = tx.send(());
                return (StatusCode::OK, String::from("Starting update..."));
            } else {
                printlg!("Update signal already sent");
                return (StatusCode::CONFLICT, String::from("Update already sent!"));
            }
        } else {
            printlg!("Already up to date!");
            return (
                StatusCode::METHOD_NOT_ALLOWED,
                String::from("Already up to date!"),
            );
        }
    } else {
        printlg!("Failed to make request to {}", REMOTE_CARGO_TOML_URL);
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("Failed to make request to {}", REMOTE_CARGO_TOML_URL),
        );
    }
}

pub async fn update_menu_handler() -> impl IntoResponse {
    if *OUT_OF_DATE.lock().await {
        return Html::from(format!("
        <div class=\"settings-box\">
            <div class=\"settings-title\">
                <h2>Update</h2>
                <div class=\"subtext-settings\">
                    <h6>Update availible! ({} -> {})</h6>
                    <h6>Do not restart the server during update!</h6>
                </div>
            </div>
            <div class=\"setting\">
                <button hx-post=\"/update\" hx-confirm=\"You are trying to update Froggi, are you sure? This completely stops Froggi until the update is complete.\">Update now</button>
            </div>
        </div>
        ", 
        env!("CARGO_PKG_VERSION"),
        *REMOTE_VERSION.lock().await
    ));
    } else {
        return Html::from(String::from("<div class=\"settings-box\"><div class=\"settings-title\"><h2>Update</h2><div class=\"subtext-settings\"><h6>Froggi is up to date!</h6></div></div></div>"));
    }
}
