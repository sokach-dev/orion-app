// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod model;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    utils::init_log();

    tauri::Builder::default()
        .manage(model::DbConnection::new(None).await.unwrap())
        .invoke_handler(tauri::generate_handler![
            service::get_review_words,
            service::learn_word,
            service::add_review_word,
            service::get_dialogs,
            service::add_new_dialog,
            service::get_vocabulary_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
