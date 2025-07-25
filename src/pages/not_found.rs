use leptos::prelude::*;
use thaw::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Flex
            class="page-not-found-message"
            style="height: 100%;"
            align=FlexAlign::Center
            justify=FlexJustify::Center
        >
            <Text style="font-size: 8rem;">404 - Not Found</Text>
        </Flex>
    }
}
