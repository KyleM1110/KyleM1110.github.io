use std::string::ParseError;

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::Deserialize;
use thaw::*;

use crate::{errors::LocalResourceError, utils};

#[derive(Deserialize, Clone, Default, Debug)]
struct DataManifest {
    slugs: Vec<String>,
}
#[derive(Deserialize, Clone, Default, Debug)]
struct MusicEntry {
    title: String,
    description: String,
    jwpepper_stars: f32,
    jwpepper_url: String,
    preview_url: String,
    last_synced: DateTime<Utc>,
}

#[component]
pub fn Music() -> impl IntoView {
    // TODO: potentially make this into a OnceResource
    let manifest_resource = utils::fetch_local_resource::<DataManifest>("/data/manifest.json");
    let mut music_resources: Vec<LocalResource<Result<MusicEntry, String>>> = Vec::new();
    let (manifest, set_manifest) = signal(Ok(String::default()));
    view! {
        <Transition>
            {move || Suspend::new(async move {
                set_manifest(
                    manifest_resource
                        .await
                        .map(|v| format!("{:?}", v.slugs))
                        .map_err(|e| LocalResourceError::FailedToFetch(
                            "/data/manifest.json".to_string(),
                        )),
                );
                view! {
                    <ErrorBoundary fallback=|errors| {
                        view! { "An error occurred" }
                    }>{manifest}</ErrorBoundary>
                }
            })}
        </Transition>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));
    let (a, _) = signal::<Result<String, ParseError>>(Ok(String::new()));
    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input
                type="number"
                on:input:target=move |ev| { set_value.set(ev.target().value().parse::<i32>()) }
            /> // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>
                <p>
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    {a} // if it is `Err`. It's a signal, so this will dynamically
                    "You entered " // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}
