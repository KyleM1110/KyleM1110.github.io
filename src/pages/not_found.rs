use leptos::prelude::*;
use thaw::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Flex style="height: 100%;" align=FlexAlign::Center justify=FlexJustify::Center>
            <Text style="font-size: 8rem;">404 - Not Found</Text>
        </Flex>
    }
}
