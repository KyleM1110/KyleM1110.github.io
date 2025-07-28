use leptos::prelude::*;
use thaw::*;

#[component]
fn BackdropLight() -> impl IntoView {
    view! {
        <svg
            style="position: absolute; opacity: 0.6;"
            viewBox="0 0 800 600"
            xmlns="http://www.w3.org/2000/svg"
            class="absolute w-full h-full z-[-1] pointer-events-none"
        >
            <defs>
                <filter id="blur" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur in="SourceGraphic" stdDeviation="90"></feGaussianBlur>
                </filter>
            </defs>

            <circle cx="200" cy="200" r="150" fill="url(#grad1)" filter="url(#blur)"></circle>
            <circle cx="600" cy="300" r="180" fill="url(#grad2)" filter="url(#blur)"></circle>
            <circle cx="400" cy="500" r="120" fill="url(#grad3)" filter="url(#blur)"></circle>

            <defs>
                <radialGradient id="grad1" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#ffc3a0"></stop>
                    <stop offset="100%" stop-color="#ffafbd"></stop>
                </radialGradient>
                <radialGradient id="grad2" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#a0d9ff"></stop>
                    <stop offset="100%" stop-color="#87e0fd"></stop>
                </radialGradient>
                <radialGradient id="grad3" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#d0ffc3"></stop>
                    <stop offset="100%" stop-color="#a3f7bf"></stop>
                </radialGradient>
            </defs>
        </svg>
    }
}

#[component]
fn BackdropDark() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 800 600"
            xmlns="http://www.w3.org/2000/svg"
            class="absolute w-full h-full z-[-1] pointer-events-none"
            style="position: absolute; opacity: 0.6;"
            style:background-color="transparent"
        >
            <defs>
                <filter id="blur" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur in="SourceGraphic" stdDeviation="90"></feGaussianBlur>
                </filter>
            </defs>

            // Blurred gradient blobs
            <circle cx="200" cy="200" r="150" fill="url(#grad1)" filter="url(#blur)"></circle>
            <circle cx="600" cy="300" r="180" fill="url(#grad2)" filter="url(#blur)"></circle>
            <circle cx="400" cy="500" r="120" fill="url(#grad3)" filter="url(#blur)"></circle>

            // Radial gradients with darker, vibrant tones
            <defs>
                <radialGradient id="grad1" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#FF6B6B"></stop>
                    <stop offset="100%" stop-color="#9B1B30"></stop>
                </radialGradient>
                <radialGradient id="grad2" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#4D96FF"></stop>
                    <stop offset="100%" stop-color="#1B2A49"></stop>
                </radialGradient>
                <radialGradient id="grad3" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stop-color="#3ECF8E"></stop>
                    <stop offset="100%" stop-color="#133F32"></stop>
                </radialGradient>
            </defs>
        </svg>
    }
}

#[component]
pub fn Backdrop(#[prop(into)] theme: Signal<Theme>) -> impl IntoView {
    view! {
        <Show when=move || theme.get().name == Theme::dark().name fallback=BackdropLight>
            <BackdropDark />
        </Show>
    }
}
