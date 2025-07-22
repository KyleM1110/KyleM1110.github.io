use leptos::prelude::*;
use leptos_use::use_preferred_dark;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let is_dark_preferred = use_preferred_dark();
    let dark_mode = RwSignal::new(false);
    let theme = RwSignal::new(Theme::light());
    // TODO: should work on making this into a Memo. According to Leptos docs this is suboptimal use of an Effect
    Effect::new(move |_| {
        let dark_preferred = is_dark_preferred.get();
        if dark_preferred {
            theme.set(Theme::dark());
        }
        dark_mode.set(dark_preferred);
    });
    view! {
        <ConfigProvider theme>
            <LoadingBarProvider>
            <h1>"Kyle Manuel"</h1>
            <Text>"Welcome to my website."</Text>
            </LoadingBarProvider>
        </ConfigProvider>
    }
}
