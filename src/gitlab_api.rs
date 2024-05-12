use reqwest;
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
    let url = format!("https://gitlab.com/users/{}/calendar.json", username);
    let body = reqwest::get(&url).await?.text().await?;
    let v: HashMap<String, i32> = serde_json::from_str(&body)?;
    return Ok(v)
}

