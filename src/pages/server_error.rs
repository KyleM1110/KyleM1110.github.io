use leptos::prelude::*;
use thaw::*;

#[component]
pub fn ServerError() -> impl IntoView {
    view! {
        <Flex
            class="server-error-message"
            style="height: 100%;"
            align=FlexAlign::Center
            justify=FlexJustify::Center
        >
            <Text style="font-size: 8rem;">500 - Internal Server Error</Text>
        </Flex>
    }
}
