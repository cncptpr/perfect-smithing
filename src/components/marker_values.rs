use leptos::prelude::*;

use crate::state::State;

use super::cell_center;

/// The distance between the two markers below which their numbers no longer
/// fit side by side: the player number then drops to a second line.
const STACK_BELOW: i64 = 12;

/// The two marker numbers, each sitting on the center of its own marker's
/// cell directly under the bar: the target value in the marker's red, the
/// player value in its green. When the markers are too close to share a
/// line, the player number moves to a second one and the row grows with it.
#[component]
pub fn MarkerValues(state: RwSignal<State>) -> impl IntoView {
    view! {
        <div class="marker-values" class=("stacked", move || needs_stack(state.get()))>
            <span
                class="marker-value marker-value-target"
                style=move || value_style(state.get().target, false)
            >
                {move || state.get().target.to_string()}
            </span>
            <span
                class="marker-value marker-value-start"
                style=move || value_style(state.get().start, needs_stack(state.get()))
            >
                {move || state.get().start.to_string()}
            </span>
        </div>
    }
}

/// Whether the two markers are too close for their numbers to share a line.
fn needs_stack(state: State) -> bool {
    (state.start - state.target).abs() < STACK_BELOW
}

/// Places one number: the center of its marker's cell horizontally, the
/// top of the row vertically — `50%` is the second line once the row is
/// stacked.
fn value_style(position: i64, second_line: bool) -> String {
    let top = if second_line { "50%" } else { "0" };
    format!("left:{}%;top:{top}", cell_center(position))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_at(start: i64, target: i64) -> State {
        State {
            start,
            target,
            ..State::default()
        }
    }

    #[test]
    fn coincident_markers_stack() {
        // Both markers start out on 0, so the default render is stacked.
        assert!(needs_stack(State::default()));
        assert!(needs_stack(state_at(60, 60)));
        assert!(needs_stack(state_at(61, 60)));
    }

    #[test]
    fn markers_apart_share_a_line() {
        assert!(!needs_stack(state_at(0, 150)));
        assert!(!needs_stack(state_at(60, 72)));
        assert!(!needs_stack(state_at(72, 60)));
    }

    #[test]
    fn stacking_only_depends_on_the_distance() {
        assert!(needs_stack(state_at(60, 71)));
        assert!(!needs_stack(state_at(60, 72)));
    }

    #[test]
    fn numbers_sit_on_their_cell_center() {
        // Cell 75 is the exact center of the bar.
        assert_eq!(value_style(75, false), "left:50%;top:0");
        assert_eq!(value_style(75, true), "left:50%;top:50%");
        assert_eq!(
            value_style(0, false),
            format!("left:{}%;top:0", cell_center(0))
        );
    }
}
