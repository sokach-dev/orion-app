use crate::model::learn_word::WrapLearnWordResponse;
use crate::model::DbConnection;
use chrono::{DateTime, Utc};
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
) -> Result<Response<Vec<WrapLearnWordResponse>>, ()> {
    match conn.get_review_words(dt).await {
        Ok(words) => wrap_success_response("success".to_string(), Some(words)),
        Err(e) => wrap_failed_response(e.to_string(), None),
    }
}

#[tauri::command]
pub async fn hello() -> String {
    "hello".to_string()
}
