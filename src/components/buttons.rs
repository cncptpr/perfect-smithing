use leptos::prelude::*;

use crate::colors::class_for;
use crate::state::State;

/// The grid the hit buttons fill, row by row; the bottom row holds the
/// empty and the clear button with a free cell at the left, which the theme
/// lays out as four centered columns.
const BUTTON_ROWS: [[i64; 4]; 2] = [[-3, -6, 2, 7], [-5, -15, 13, 16]];

/// The eight hit buttons in their grid plus the bottom row. A hit fills the
/// selected slot and advances the selection, wrapping around; the empty
/// button clears the selected slot and advances the selection the same way,
/// while `C` empties every slot and sends the selection back to the first.
#[component]
pub fn HitButtons(state: RwSignal<State>) -> impl IntoView {
    view! {
        <div class="hit-buttons">
            {BUTTON_ROWS
                .into_iter()
                .flatten()
                .map(|step| {
                    view! {
                        <button
                            type="button"
                            class=format!("hit-btn {}", class_for(step))
                            on:click=move |_| state.update(|current| current.fill_selected(step))
                        >
                            {step.to_string()}
                        </button>
                    }
                })
                .collect_view()}
            <button
                type="button"
                class="hit-btn empty-btn"
                on:click=move |_| state.update(|current| current.clear_selected())
            >
                "empty"
            </button>
            <button
                type="button"
                class="hit-btn clear-btn"
                title="Clear all"
                on:click=move |_| state.update(|current| current.clear_all())
            >
                "C"
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::DEFAULT_STEPS;

    #[test]
    fn the_grid_holds_every_hit_exactly_once() {
        let mut grid: Vec<i64> = BUTTON_ROWS.into_iter().flatten().collect();
        let mut steps = DEFAULT_STEPS.to_vec();
        grid.sort_unstable();
        steps.sort_unstable();

        assert_eq!(grid, steps);
    }

    #[test]
    fn the_first_row_reads_minus3_minus6_2_7() {
        assert_eq!(BUTTON_ROWS[0], [-3, -6, 2, 7]);
    }

    #[test]
    fn the_second_row_reads_minus5_minus15_13_16() {
        assert_eq!(BUTTON_ROWS[1], [-5, -15, 13, 16]);
    }
}
