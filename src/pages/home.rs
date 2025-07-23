use std::marker::PhantomData;

use leptos::prelude::*;
use thaw::*;

use crate::components::LinkButton;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Flex justify=FlexJustify::End>
            <Image
                attr:style="position: absolute; z-index: 0; padding-right: 5%; opacity: 1;"
                shape=ImageShape::Rounded
                src="/images/kyle.png"
                height="110%"
            />
        </Flex>
        <Flex
            style="padding: 4%; height: 100%; box-sizing: border-box;"
            class="home-page"
            justify=FlexJustify::SpaceBetween
        >
            <Flex class="introduction" style="z-index: 1; width: 40%;" vertical=true>
                <Text style="font-size: 5rem; line-height: 5.5rem;" tag=TextTag::H1>
                    <strong>"Hello, I'm Kyle."</strong>
                </Text>
                <Body1 style="font-size: 2rem; line-height: 2.5rem;">
                    "I'm a trombonist, elementary school music teacher, assistant band director, and composer."
                </Body1>
            </Flex>
            <Flex
                style="padding: 0 10%; z-index: 1;"
                vertical=true
                align=FlexAlign::Center
                justify=FlexJustify::End
            >
                <LinkButton href="/music" size=ButtonSize::Large>
                    "View My Work"
                </LinkButton>
            </Flex>
        </Flex>
    }
}
