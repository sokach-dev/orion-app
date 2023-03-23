// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use time::{macros::format_description, UtcOffset};
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};

mod errors;
mod model;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt()
        .with_timer(local_time)
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .with_line_number(true)
        .with_file(true)
        .init();

    tauri::Builder::default()
        .manage(model::DbConnection::new(None, None).await.unwrap())
        .invoke_handler(tauri::generate_handler![
            service::get_review_words,
            service::hello
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
