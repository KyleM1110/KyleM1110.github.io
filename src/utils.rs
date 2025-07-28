use anyhow::{bail, Context};
use chrono::{DateTime, Utc};
use gloo_net::http::{Request, Response};
use js_sys::{wasm_bindgen::JsValue, Date};
use leptos::{prelude::window, server::LocalResource};
use serde::de::DeserializeOwned;

use crate::errors::LocalResourceError;

async fn get_local_response(url: &str) -> anyhow::Result<Response> {
    let url = url.trim().trim_start_matches('/');
    let url = format!("{}/{url}", window().location().origin().unwrap());
    log::debug!("Fetching local resource: '{url}'");
    let response = Request::get(&url).send().await.context(format!(
        "Failed to send request to local resource: '{}'",
        &url
    ))?;
    if !response.ok() {
        bail!(
            "Response to fetch to local resource '{}' was not successful",
            url
        );
    }
    Ok(response)
}
pub async fn fetch_local_raw(url: &str) -> anyhow::Result<String> {
    let response = get_local_response(url).await?;
    let body = response
        .text()
        .await
        .context("Failed to read response as a string")?;
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

pub fn get_local_timestamp(timestamp: DateTime<Utc>) -> String {
    let js_date = Date::new(&timestamp.to_string().into());
    js_date
        .to_locale_string("default", &JsValue::undefined())
        .into()
}
