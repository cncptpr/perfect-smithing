use leptos::prelude::*;

use crate::solve::DEFAULT_MAX_POS;

use super::cell_center;

/// The number line above the bar: `0, 20, …, 180`, every number sitting on
/// the center of its cell.
#[component]
pub fn Numbers() -> impl IntoView {
    view! {
        <div class="numbers">
            {(0..=DEFAULT_MAX_POS)
                .step_by(20)
                .map(|value| {
                    view! {
                        <span class="number" style=format!("left:{}%", cell_center(value))>
                            {value.to_string()}
                        </span>
                    }
                })
                .collect_view()}
        </div>
    }
}
