pub mod apsp;
pub mod cache;
pub mod colors;
pub mod matrix;
pub mod num;
pub mod solve;
pub mod state;
pub mod url;

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod app;

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod components;

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod web_solve;

pub use apsp::{APSPResult, floyd_warshall};
pub use cache::{load_cache, store_cache};
pub use matrix::Matrix;
pub use num::Num;
pub use solve::{CoreError, DEFAULT_MAX_POS, DEFAULT_STEPS, LastHits, Slot, Solution, solve};

/// Entry point called by the wasm bundle to take over the server rendered body.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
