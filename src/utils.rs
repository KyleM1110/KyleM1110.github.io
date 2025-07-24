use anyhow::Context;
use leptos::{prelude::window, server::LocalResource};
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::errors::LocalResourceError;

async fn get_local_response(url: &str) -> anyhow::Result<Response> {
    let url = url.trim().trim_start_matches('/');
    let url = format!("{}/{url}", window().location().origin().unwrap());
    log::debug!("Fetching local resource: '{url}'");
    let response = reqwest::get(&url)
        .await
        .context(format!("Failed to fetch local resource: '{}'", &url))?
        .error_for_status()
        .context("Request to retrieve local resource did not have a successful response")?;
    Ok(response)
}
pub async fn fetch_local_raw(url: &str) -> anyhow::Result<String> {
    let response = get_local_response(url).await?;
    let body = response
        .text()
        .await
        .context("Failed to read response body")?;
    Ok(body)
}

pub fn fetch_local_resource<T>(url: String) -> LocalResource<Result<T, LocalResourceError>>
where
    T: DeserializeOwned + 'static,
{
    LocalResource::new(move || {
        let value = url.clone();
        async move {
            let data = fetch_local_raw(&value)
                .await
                .map(|raw_data| {
                    serde_json::from_str::<T>(&raw_data)
                        .context(format!("Failed to serialize '{value}'"))
                })
                .flatten()
                .map_err(|e| LocalResourceError::FailedToFetch(e.to_string()))
                .inspect(|_| log::debug!("Successfully retrieved local data from '{value}'"));
            data
        }
    })
}
