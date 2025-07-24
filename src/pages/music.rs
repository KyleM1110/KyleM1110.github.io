use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::Deserialize;
use thaw::*;

use crate::{errors::LocalResourceError, pages::ServerError, utils};

const MANIFEST_URL: &str = "/data/manifest.json";
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
fn MusicEntryCard(entry: MusicEntry) -> impl IntoView {
    view! { {entry.title} }
}

#[component]
pub fn Music() -> impl IntoView {
    // TODO: potentially make this into a OnceResource
    let manifest_resource = utils::fetch_local_resource::<DataManifest>(MANIFEST_URL.to_string());
    let (entry_resources, _) =
        signal(Vec::<LocalResource<Result<MusicEntry, LocalResourceError>>>::new());
    view! {
        <Transition>
            {move || Suspend::new(async move {
                let manifest = manifest_resource.await;
                if let Ok(ref m) = manifest {
                    let slugs = m.slugs.clone();
                    for slug in slugs {
                        entry_resources()
                            .push(utils::fetch_local_resource::<MusicEntry>(slug.to_string()))
                    }
                }
                log::debug!(
                    "Created {} local resources for music entries", entry_resources().len()
                );
                let mut entries = Vec::<Result<MusicEntry, LocalResourceError>>::new();
                for resource in entry_resources() {
                    entries.push(resource.await);
                }
                log::info!("Loaded {} music entries", entries.len());
                let grid_items = entries
                    .into_iter()
                    .map(|result| {
                        result
                            .map(|entry| {
                                view! {
                                    <GridItem>
                                        <MusicEntryCard entry />
                                    </GridItem>
                                }
                            })
                    })
                    .collect_view();
                view! {
                    <ErrorBoundary fallback=|errors| {
                        for (_, error) in errors() {
                            log::error!("{}", error);
                        }
                        view! { <ServerError /> }
                    }>
                        {manifest.map(|_| ())} <Grid class="music-entry-cards" cols=3>
                            {grid_items}
                        </Grid>
                    </ErrorBoundary>
                }
            })}
        </Transition>
    }
}
