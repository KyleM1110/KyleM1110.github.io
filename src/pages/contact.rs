use leptos::prelude::*;
use thaw::*;

const EMAIL: &str = "";

#[component]
pub fn Contact() -> impl IntoView {
    // TODO: work on getting this into a form. Something like Formspree, Getform, or something potentially.
    let href = format!("mailto:{}", EMAIL);
    view! {
        <Flex vertical=true class="contact-page" style="height: calc(100% - 40px); padding: 20px;">
            <Text style="font-size: 2rem;" tag=TextTag::H1>
                "Contact Me"
            </Text>
            <Body1>"Feel free to get in touch with me at " <Link href>{EMAIL}</Link>"."</Body1>
        </Flex>
    }
}
