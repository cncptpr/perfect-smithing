use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};

use crate::components::{
    Arrows, Bar, HitButtons, MarkerInputs, MarkerValues, Numbers, Slots, SolutionLine,
};
use crate::state::State;
use crate::web_solve;

#[cfg(feature = "hydrate")]
use crate::url;

/// The document the SSR server renders, wrapping [`App`].
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let state = RwSignal::new(initial_state());
    let solution = web_solve::solution_memo(state);
    #[cfg(feature = "hydrate")]
    restore_url(state);
    #[cfg(feature = "hydrate")]
    sync_url(state);

    view! {
        // id=leptos lets cargo-leptos hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/perfect_smithing.css"/>
        <Title text="Perfect Smithing"/>
        <main>
            <h1>"Perfect Smithing"</h1>
            <Slots state/>
            <HitButtons state/>
            <MarkerInputs state/>
            <div class="gauge">
                <Numbers/>
                <Bar state/>
                <MarkerValues state/>
                <Arrows solution/>
            </div>
            <SolutionLine solution/>
        </main>
    }
}

/// The state the very first render starts from. Hydration has to reproduce
/// the server's markup exactly, so this is the defaults on both sides; the
/// browser applies the address bar right afterwards, in [`restore_url`].
fn initial_state() -> State {
    State::default()
}

/// Applies the address bar once hydration is done. The first render already
/// matched the server with the defaults, so this effect (running after that
/// synchronous work, on the first tick) may safely move the state to the
/// shared URL — its patches are ordinary updates, not hydration.
#[cfg(feature = "hydrate")]
fn restore_url(state: RwSignal<State>) {
    let restored = url::from_query(&location().search().unwrap_or_default());
    if restored != State::default() {
        Effect::new(move |_| state.set(restored));
    }
}

/// Keeps the address bar and the state in step: every change rewrites the URL
/// with `replaceState` (so going back skips intermediate states) and `popstate`
/// parses history entries back into the state.
#[cfg(feature = "hydrate")]
fn sync_url(state: RwSignal<State>) {
    Effect::new(move |_| {
        let query = url::to_query(&state.get());
        let path = location().pathname().unwrap_or_default();
        if let Ok(history) = window().history() {
            _ = history.replace_state_with_url(
                &wasm_bindgen::JsValue::NULL,
                "",
                Some(&format!("{path}{query}")),
            );
        }
    });

    let handle = window_event_listener(leptos::ev::popstate, move |_| {
        let search = location().search().unwrap_or_default();
        state.set(url::from_query(&search));
    });
    on_cleanup(move || handle.remove());
}
