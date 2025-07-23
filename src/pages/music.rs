use leptos::prelude::*;
use thaw::*;

use crate::components::LinkButton;

async fn fetch_monkeys() -> String {
    // maybe this didn't need to be async
    let url = format!("{}/data/test.json", window().location().origin().unwrap());
    log::info!("{}", url);
    let body = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return body;
}

#[component]
pub fn Music() -> impl IntoView {
    let async_data = LocalResource::new(|| fetch_monkeys());
    view! {
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            // Suspend allows you use to an async block in the view
            <p>
                "Your test data is "
                {move || Suspend::new(async move {
                    async_data.await
                })}
            </p>
        </Suspense>
    }
}
