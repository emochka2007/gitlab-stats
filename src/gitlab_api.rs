use reqwest;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub async fn fetch_gitlab_stats(username: String) -> Result<HashMap<String, i32>,FetchError>{
    let body = reqwest::get("https://gitlab.com/users/emochka2007/calendar.json")
        .await?
        .text()
        .await?;

    let v: HashMap<String, i32> = serde_json::from_str(&body)?;
    return Ok(v)
}

