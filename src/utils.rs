use anyhow::{Context, Result};
use leptos::prelude::window;
use reqwest::Response;
use serde::de::DeserializeOwned;

async fn get_local_response(url: &str) -> Result<Response> {
    let url = url.trim().trim_start_matches('/');
    let url = format!("{}/{url}", window().location().origin().unwrap());
    log::debug!("Fetching local resource: '{url}'");
    let response = reqwest::get(&url)
        .await
        .context(format!("Failed to fetch local resource: '{}'", &url))?
        .error_for_status()
        .context("Response was not successful")?;
    Ok(response)
}
pub async fn fetch_local_raw(url: &str) -> Result<String> {
    let response = get_local_response(url).await?;
    let body = response
        .text()
        .await
        .context("Failed to read response body")?;
    Ok(body)
}

pub async fn fetch_local<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let response = get_local_response(url).await?;
    let body = response
        .json::<T>()
        .await
        .context("Failed to deserialize JSON response")?;
    Ok(body)
}
