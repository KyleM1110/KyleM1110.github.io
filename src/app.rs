use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use leptos_use::use_preferred_dark;
use thaw::*;

use crate::{components::*, pages::*};

#[component]
fn NavButton(#[prop(into, optional)] href: &'static str, children: Children) -> impl IntoView {
    view! {
        <LinkButton class="nav-button" appearance=ButtonAppearance::Transparent href>
            {children()}
        </LinkButton>
    }
}

#[component]
fn NavBar(theme: RwSignal<Theme>) -> impl IntoView {
    let icon = Memo::new(move |_| {
        if theme.get().name == Theme::light().name {
            icondata_ai::AiMoonOutlined
        } else {
            icondata_ai::AiSunOutlined
        }
    });
    let text = Memo::new(move |_| {
        if theme.get().name == Theme::light().name {
            "Dark"
        } else {
            "Light"
        }
    });
    view! {
        <Flex class="nav-bar" style="height: 100%; padding-left: 13px;" style:background-color=move || theme.get().color.color_neutral_background_3 align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
            <Text style="font-size: 1.5rem;" tag=TextTag::H1>
                <b>"Kyle Manuel"</b>
            </Text>
            <Flex gap=FlexGap::Small>
                <Button
                    class="toggle-theme-button"
                    icon
                    on_click=move |_| theme.set(if theme.get().name == Theme::light().name { Theme::dark() } else { Theme::light() })
                >
                    {text}
                </Button>
                <NavButton href="/home">"Home"</NavButton>
                <NavButton href="/music">"Music"</NavButton>
                <NavButton href="/contact">"Contact"</NavButton>
            </Flex>
        </Flex>
        <Divider />
    }
}

#[component]
fn Footer(#[prop(into)] theme: Signal<Theme>) -> impl IntoView {
    view! {
        <Divider />
        <Flex
            style:background-color=move || theme.get().color.color_neutral_background_3
            style="height: 100%;"
            class="footer"
            vertical=true
            align=FlexAlign::Center
            justify=FlexJustify::Center
        >
            <Caption1Strong class="copyright">
                "© 2025 Kyle Manuel & Dylan Manuel. All rights reserved."
            </Caption1Strong>
            <Caption1 class="credits">
                "Created by " <Link href="https://dmanuel64.github.io">"Dylan Manuel"</Link>
            </Caption1>
        </Flex>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let dark_preferred = use_preferred_dark();
    let dark = RwSignal::new(false);
    let theme = RwSignal::new(Theme::light());
    // TODO: should work on making this into a Memo. According to Leptos docs this is suboptimal use of an Effect
    Effect::new(move |_| {
        let dark_preferred = dark_preferred.get();
        if dark_preferred {
            theme.set(Theme::dark());
        }
        dark.set(dark_preferred);
    });
    view! {
        <Router>
            <ConfigProvider theme>
                <LoadingBarProvider>
                    <Layout
                        class="root"
                        content_style="height: 100vh;"
                        position=LayoutPosition::Absolute
                    >
                        <LayoutHeader>
                            <nav style="height: 10vh; z-index: 1;">
                                <NavBar theme />
                            </nav>
                        </LayoutHeader>
                        <Layout
                            attr:style="top: 10vh; height: 90vh;"
                            position=LayoutPosition::Absolute
                        >
                            <Backdrop theme />
                            <Flex
                                gap=FlexGap::Size(0)
                                style="min-height: 90vh;"
                                vertical=true
                                justify=FlexJustify::SpaceBetween
                            >
                                <main style="height: 80vh;">
                                    <Routes fallback=NotFound>
                                        <Route path=path!("/") view=Home />
                                        <Route path=path!("/home") view=Home />
                                        <Route path=path!("/music") view=Music />
                                    </Routes>
                                </main>
                                <footer style="height: 10vh; width: 100%; z-index: 1;">
                                    <Footer theme />
                                </footer>
                            </Flex>
                        </Layout>
                    </Layout>
                </LoadingBarProvider>
            </ConfigProvider>
        </Router>
    }
}
