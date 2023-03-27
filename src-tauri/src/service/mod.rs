use crate::model::{
    data_scaffold::{dialog::Dialog, learn_word::LearnWord},
    DbConnection,
};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseStatus {
    Success,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    status: ResponseStatus,
    msg: String,
    data: Option<T>,
}

pub fn wrap_response<T>(
    status: ResponseStatus,
    msg: String,
    data: Option<T>,
) -> Result<Response<T>, ()> {
    Ok(Response { status, msg, data })
}

pub fn wrap_success_response<T>(msg: String, data: Option<T>) -> Result<Response<T>, ()> {
    wrap_response(ResponseStatus::Success, msg, data)
}

pub fn wrap_failed_response<T>(msg: String, data: Option<T>) -> Result<Response<T>, ()> {
    wrap_response(ResponseStatus::Failed, msg, data)
}

#[tauri::command]
pub async fn get_review_words(
    conn: State<'_, DbConnection>,
    dt: String, // 2022-01-01
) -> Result<Response<Vec<LearnWord>>, ()> {
    match conn.get_review_words(dt).await {
        Ok(words) => wrap_success_response("success".to_string(), Some(words)),
        Err(e) => wrap_failed_response(e.to_string(), None),
    }
}

#[tauri::command]
pub async fn add_review_word(
    conn: State<'_, DbConnection>,
    word: String,
) -> Result<Response<()>, ()> {
    match conn.add_review_word(word).await {
        Ok(_) => wrap_success_response("success".to_string(), None),
        Err(e) => wrap_failed_response(e.to_string(), None),
    }
}

#[tauri::command]
pub async fn get_dialogs(
    conn: State<'_, DbConnection>,
    page: u32, // page number
    size: u32, // page size
) -> Result<Response<Vec<Dialog>>, ()> {
    match conn.get_dialogs(size, page).await {
        Ok(dialogs) => wrap_success_response("success".to_string(), Some(dialogs)),
        Err(e) => wrap_failed_response(e.to_string(), None),
    }
}

#[tauri::command]
pub async fn learn_word(
    conn: State<'_, DbConnection>,
    id: i64,      // word_id
    count: i64,   // learn_count
    next: String, // next_learn_at
    status: u8,   // 0: 忘了, 1: 看答案了, 4: 记住了
) -> Result<Response<()>, ()> {
    match conn.learn_word(id, count, next, status).await {
        Ok(_) => wrap_success_response("success".to_string(), None),
        Err(e) => wrap_failed_response(e.to_string(), None),
    }
}
