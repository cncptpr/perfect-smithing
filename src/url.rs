use crate::solve::{DEFAULT_MAX_POS, DEFAULT_STEPS};
use crate::state::State;

/// The state as a shareable query string, e.g.
/// `?start=0&target=60&last=16&second=13&third=7&sel=1`.
/// Defaults are left out, so the default state yields an empty string.
pub fn to_query(state: &State) -> String {
    let mut params = Vec::new();

    if state.start != 0 {
        params.push(format!("start={}", state.start));
    }
    if state.target != 0 {
        params.push(format!("target={}", state.target));
    }
    for (name, slot) in ["last", "second", "third"].into_iter().zip(state.slots) {
        if let Some(value) = slot {
            params.push(format!("{name}={value}"));
        }
    }
    if state.selected != 0 {
        params.push(format!("sel={}", state.selected));
    }

    if params.is_empty() {
        String::new()
    } else {
        format!("?{}", params.join("&"))
    }
}

/// Parses a query string, with or without the leading `?`, back into a state.
/// Unknown keys and malformed values are dropped, markers are clamped to the
/// bar, slot values outside [`DEFAULT_STEPS`] are dropped and `sel` is clamped
/// to the slots. When a key repeats, the last valid value wins.
pub fn from_query(query: &str) -> State {
    let mut state = State::default();

    for pair in query.strip_prefix('?').unwrap_or(query).split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };

        match key {
            "start" => parse_bounded(&mut state.start, value),
            "target" => parse_bounded(&mut state.target, value),
            "last" => parse_slot(&mut state.slots[0], value),
            "second" => parse_slot(&mut state.slots[1], value),
            "third" => parse_slot(&mut state.slots[2], value),
            "sel" => {
                if let Ok(selected) = value.parse::<usize>() {
                    state.selected = selected.min(state.slots.len() - 1);
                }
            }
            _ => {}
        }
    }

    state
}

fn parse_bounded(marker: &mut i64, value: &str) {
    if let Ok(value) = value.parse::<i64>() {
        *marker = value.clamp(0, DEFAULT_MAX_POS);
    }
}

fn parse_slot(slot: &mut Option<i64>, value: &str) {
    if let Ok(value) = value.parse::<i64>()
        && DEFAULT_STEPS.contains(&value)
    {
        *slot = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omits_defaults_and_writes_a_stable_order() {
        assert_eq!(to_query(&State::default()), "");

        let partial = State {
            target: 60,
            ..State::default()
        };
        assert_eq!(to_query(&partial), "?target=60");

        let full = State {
            start: 5,
            target: 60,
            slots: [Some(16), None, Some(7)],
            selected: 2,
        };
        assert_eq!(to_query(&full), "?start=5&target=60&last=16&third=7&sel=2");
    }

    #[test]
    fn round_trips_a_full_state() {
        let state = State {
            start: 5,
            target: 60,
            slots: [Some(16), Some(13), Some(7)],
            selected: 1,
        };

        assert_eq!(
            to_query(&state),
            "?start=5&target=60&last=16&second=13&third=7&sel=1"
        );
        assert_eq!(from_query(&to_query(&state)), state);
    }

    #[test]
    fn parses_without_the_question_mark() {
        let expected = State {
            target: 60,
            ..State::default()
        };

        assert_eq!(from_query("target=60"), expected);
        assert_eq!(from_query(""), State::default());
        assert_eq!(from_query("?"), State::default());
    }

    #[test]
    fn drops_junk_and_clamps_out_of_range_values() {
        let state =
            from_query("?start=abc&start=7&target=999&last=17&second=-6&third=2&sel=9&junk=1&=2");

        assert_eq!(
            state,
            State {
                start: 7,                         // "start=abc" is not a number
                target: 150,                      // clamped to the bar
                slots: [None, Some(-6), Some(2)], // 17 is not one of the hits
                selected: 2,                      // clamped to the third slot
            }
        );
    }

    #[test]
    fn ignores_unparsable_values() {
        assert_eq!(
            from_query("?start=99999999999999999999999&target=&sel=-1&&&"),
            State::default()
        );
    }
}
