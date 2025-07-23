use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::*;

#[component]
pub fn LinkButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A button can have its content and borders styled for greater emphasis or to be subtle.
    #[prop(optional, into)]
    appearance: Signal<ButtonAppearance>,
    /// A button can be rounded, circular, or square.
    #[prop(optional, into)]
    shape: Signal<ButtonShape>,
    /// A button supports different sizes.
    #[prop(optional, into)]
    size: Signal<ButtonSize>,
    /// The default behavior of the button.
    #[prop(optional, into)]
    button_type: MaybeProp<ButtonType>,
    /// Whether the button is displayed as block.
    #[prop(optional, into)]
    block: Signal<bool>,
    /// The icon of the button.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Whether the button is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// When set, allows the button to be focusable even when it has been disabled.
    #[prop(optional, into)]
    disabled_focusable: Signal<bool>,
    /// Whether the button shows the loading status.
    #[prop(optional, into)]
    loading: Signal<bool>,
    #[prop(optional)] comp_ref: ComponentRef<ButtonRef>,
    #[prop(optional, into)] href: &'static str,
    children: Children,
) -> impl IntoView {
    let navigate = use_navigate();
    view! {
        <Button
            class
            appearance
            shape
            size
            button_type
            block
            icon
            disabled
            disabled_focusable
            loading
            comp_ref
            on_click=move |_| {
                if !href.is_empty() {
                    navigate(href, Default::default())
                }
            }
        >
            <Link attr:style="color: inherit; text-decoration: none;" href=href>
                {children()}
            </Link>
        </Button>
    }
}
