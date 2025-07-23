use leptos::prelude::*;
use thaw::*;

use crate::components::LinkButton;

async fn fetch_monkeys() -> String {
    // maybe this didn't need to be async
    let body = reqwest::get("http://[::1]:8080/images/kyle.png")
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
                "Your shouting name is "
                {move || Suspend::new(async move {
                    async_data.await
                })}
            </p>
        </Suspense>
    }
}
