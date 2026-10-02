use leptos::prelude::*;

use crate::solve::DEFAULT_MAX_POS;
use crate::state::State;

/// Two number fields for the marker positions, for when tapping an exact
/// cell on the bar gets annoying on mobile or narrow screens. They sit
/// between the button panel and the bar, carry the markers' colours and
/// mirror the state: anything that does not read as a plain number is not
/// applied, and the setters clamp every position onto the bar.
#[component]
pub fn MarkerInputs(state: RwSignal<State>) -> impl IntoView {
    view! {
        <div class="marker-inputs">
            <label class="marker-input marker-input-start">
                <span class="marker-input-label">"player"</span>
                <input
                    type="number"
                    min="0"
                    max=DEFAULT_MAX_POS.to_string()
                    inputmode="numeric"
                    prop:value=move || state.get().start.to_string()
                    on:input=move |ev| {
                        if let Some(position) = parse_position(&event_target_value(&ev)) {
                            state.update(|current| current.set_start(position));
                        }
                    }
                />
            </label>
            <label class="marker-input marker-input-target">
                <span class="marker-input-label">"target"</span>
                <input
                    type="number"
                    min="0"
                    max=DEFAULT_MAX_POS.to_string()
                    inputmode="numeric"
                    prop:value=move || state.get().target.to_string()
                    on:input=move |ev| {
                        if let Some(position) = parse_position(&event_target_value(&ev)) {
                            state.update(|current| current.set_target(position));
                        }
                    }
                />
            </label>
        </div>
    }
}

/// The field's contents as a position; `None` while it does not read as a
/// plain number, so an emptied field simply waits for the next digits
/// instead of being rewritten mid-edit.
fn parse_position(text: &str) -> Option<i64> {
    text.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_numbers_parse() {
        assert_eq!(parse_position("0"), Some(0));
        assert_eq!(parse_position("60"), Some(60));
        assert_eq!(parse_position("180"), Some(180));
        assert_eq!(parse_position(" 42 "), Some(42));
        // Out-of-range values parse too; the setters clamp them onto the bar.
        assert_eq!(parse_position("999"), Some(999));
        assert_eq!(parse_position("-5"), Some(-5));
    }

    #[test]
    fn anything_else_waits() {
        assert_eq!(parse_position(""), None);
        assert_eq!(parse_position("   "), None);
        assert_eq!(parse_position("6.5"), None);
        assert_eq!(parse_position("abc"), None);
        assert_eq!(parse_position("1e3"), None);
    }

    #[test]
    fn parsed_positions_end_up_on_the_markers() {
        let mut state = State::default();

        state.set_target(parse_position("179").unwrap());
        state.set_start(parse_position("999").unwrap());

        assert_eq!((state.start, state.target), (180, 179));
    }
}
