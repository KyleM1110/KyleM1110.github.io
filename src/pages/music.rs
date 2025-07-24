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
    // TODO: this is very hacky, but it seems like this is the only way to store a vector of resources outside the view
    let (entry_resources, set_entry_resources) =
        signal(Vec::<LocalResource<Result<MusicEntry, LocalResourceError>>>::new());
    // TODO: need to add pages. This isn't immediately until more music is published
    view! {
        <Transition>
            {move || Suspend::new(async move {
                let manifest = manifest_resource.await;
                if let Ok(ref m) = manifest {
                    let slugs = m.slugs.clone();
                    for slug in slugs {
                        set_entry_resources({
                            let mut v = entry_resources();
                            v.push(utils::fetch_local_resource::<MusicEntry>(slug.to_string()));
                            v
                        });
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
                view! {
                    <ErrorBoundary fallback=|errors| {
                        for (_, error) in errors() {
                            log::error!("{}", error);
                        }
                        view! { <ServerError /> }
                    }>
                        {manifest.map(|_| ())}
                        <Flex
                            style="height: 100%;"
                            class="music-entry-cards"
                            align=FlexAlign::Center
                            justify=FlexJustify::Center
                        >
                            <Grid class="music-entry-cards-grid" cols=3>
                                {entries
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
                                    .collect_view()}
                            </Grid>
                        </Flex>
                    </ErrorBoundary>
                }
            })}
        </Transition>
    }
}
