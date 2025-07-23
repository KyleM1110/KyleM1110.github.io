use leptos::prelude::*;
use thaw::*;

async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
    monkey * 2
}

#[component]
pub fn LocalData(#[prop(into)] url: String) -> impl IntoView {
    view! {}
}
