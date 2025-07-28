use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::Deserialize;
use thaw::*;

use crate::{components::LinkButton, errors::LocalResourceError, pages::ServerError, utils};

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
fn MusicEntryCard(entry: MusicEntry, theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <Flex
            class="music-entry-card"
            style="border-radius: 25px;"
            vertical=true
            style:background-color=move || { theme.get().color.color_neutral_background_4 }
        >
            <Flex class="music-entry-card-header" style="padding: 20px;" vertical=true>
                <Text tag=TextTag::H1>
                    <strong>{entry.title}</strong>
                </Text>
                <Caption1>{entry.description}</Caption1>
                <RatingDisplay class="music-entry-rating" value=entry.jwpepper_stars />
            </Flex>
            <Divider />
            <Flex
                class="music-entry-card-preview-container"
                style="padding: 20px;"
                vertical=true
                align=FlexAlign::Center
                justify=FlexJustify::Center
            >
                <iframe
                    class="music-entry-card-preview"
                    width="280"
                    height="158"
                    src=entry.preview_url
                    title="YouTube video player"
                    style:border="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen
                ></iframe>
            </Flex>
            <Divider />
            <Flex class="music-entry-card-footer-container" style="padding: 20px;" vertical=true>
                <Flex
                    class="music-entry-card-footer"
                    justify=FlexJustify::SpaceBetween
                    align=FlexAlign::Center
                >
                    <Caption1>
                        {format!("Last synced: {}", utils::get_local_timestamp(entry.last_synced))}
                    </Caption1>
                    <LinkButton shape=ButtonShape::Rounded href=entry.jwpepper_url>
                        "More Details"
                    </LinkButton>
                </Flex>
            </Flex>
        </Flex>
    }
}

#[component]
pub fn Music(#[prop(into)] theme: RwSignal<Theme>) -> impl IntoView {
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
                            class="music-entry-cards-container"
                            align=FlexAlign::Center
                            justify=FlexJustify::Center
                        >
                            <Grid class="music-entry-cards-grid" x_gap=20 y_gap=20 cols=2>
                                {entries
                                    .into_iter()
                                    .map(|result| {
                                        result
                                            .map(|entry| {
                                                view! {
                                                    <GridItem>
                                                        <MusicEntryCard entry theme />
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
