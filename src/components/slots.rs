use leptos::prelude::*;

use crate::colors::class_for;
use crate::state::State;

/// The three finishing slots in game order, read left to right.
const SLOT_LABELS: [&str; 3] = ["last", "second", "third"];

/// The slot row: the hit buttons fill the selected slot, clicking a slot
/// selects it. The selected slot carries the `selected` class the theme turns
/// into an outline, a filled slot shows its value in the hit colour and an
/// empty one shows an em dash.
#[component]
pub fn Slots(state: RwSignal<State>) -> impl IntoView {
    view! {
        <div class="slots">
            {(0..SLOT_LABELS.len())
                .map(|index| {
                    view! {
                        <button
                            type="button"
                            class="slot"
                            class=("selected", move || state.get().selected == index)
                            on:click=move |_| state.update(|current| current.select(index))
                        >
                            <span class="slot-label">{SLOT_LABELS[index]}</span>
                            <span class=move || value_class(&state.get(), index)>
                                {move || value_text(&state.get(), index)}
                            </span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// The value line of one slot: the hit colour when filled, the empty class
/// when not.
fn value_class(state: &State, index: usize) -> String {
    match state.slots[index] {
        Some(value) => format!("slot-value {}", class_for(value)),
        None => "slot-value slot-empty".to_string(),
    }
}

/// What one slot shows: its value or an em dash while it is empty.
fn value_text(state: &State, index: usize) -> String {
    state.slots[index].map_or_else(|| "\u{2014}".to_string(), |value| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_filled_slot_shows_its_hit_colour() {
        let mut state = State::default();
        state.fill_selected(16);

        assert_eq!(value_class(&state, 0), "slot-value hit-16");
        assert_eq!(value_text(&state, 0), "16");
    }

    #[test]
    fn an_empty_slot_shows_its_empty_class_and_a_dash() {
        let state = State::default();

        for index in 0..SLOT_LABELS.len() {
            assert_eq!(value_class(&state, index), "slot-value slot-empty");
            assert_eq!(value_text(&state, index), "\u{2014}");
        }
    }

    #[test]
    fn the_slot_labels_read_last_second_third() {
        assert_eq!(SLOT_LABELS, ["last", "second", "third"]);
    }
}
