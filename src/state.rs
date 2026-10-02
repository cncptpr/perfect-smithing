use crate::solve::{CoreError, DEFAULT_MAX_POS, DEFAULT_STEPS, LastHits};

/// Everything the interface mutates: the two markers, the three finishing
/// slots and which slot the hit buttons write into. The slots only ever hold
/// values from [`DEFAULT_STEPS`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct State {
    /// Green player marker, default 0.
    pub start: i64,
    /// Red target marker, default 0.
    pub target: i64,
    /// The finishing slots in game order, `[last, second, third]`.
    pub slots: [Option<i64>; 3],
    /// The slot the hit buttons fill, `0..3`, default 0 (slot `last`).
    pub selected: usize,
}

impl State {
    /// Writes `value` into the selected slot and moves the selection on,
    /// wrapping around from the third slot back to the first.
    pub fn fill_selected(&mut self, value: i64) {
        self.slots[self.selected] = Some(value);
        self.selected = (self.selected + 1) % self.slots.len();
    }

    /// Empties the selected slot and moves the selection on, exactly like
    /// filling it does, wrapping around from the third slot to the first.
    pub fn clear_selected(&mut self) {
        self.slots[self.selected] = None;
        self.selected = (self.selected + 1) % self.slots.len();
    }

    /// Empties every slot and points the hit buttons at the first slot.
    pub fn clear_all(&mut self) {
        self.slots = [None; 3];
        self.selected = 0;
    }

    /// Points the hit buttons at another slot.
    pub fn select(&mut self, slot: usize) {
        self.selected = slot.min(self.slots.len() - 1);
    }

    /// Moves the green player marker, clamped to the bar.
    pub fn set_start(&mut self, position: i64) {
        self.start = position.clamp(0, DEFAULT_MAX_POS);
    }

    /// Moves the red target marker, clamped to the bar.
    pub fn set_target(&mut self, position: i64) {
        self.target = position.clamp(0, DEFAULT_MAX_POS);
    }

    /// The slots in the shape the solver expects.
    pub fn last_hits(&self) -> Result<LastHits, CoreError> {
        LastHits::from_slots(self.slots, &DEFAULT_STEPS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_the_selected_slot_and_advances_with_wrap() {
        let mut state = State::default();

        state.fill_selected(16);
        assert_eq!(state.slots, [Some(16), None, None]);
        assert_eq!(state.selected, 1);

        state.fill_selected(13);
        state.fill_selected(2);
        assert_eq!(state.slots, [Some(16), Some(13), Some(2)]);
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn clearing_advances_the_selection() {
        let mut state = State::default();
        state.fill_selected(16);
        state.fill_selected(13);
        state.select(1);

        state.clear_selected();

        assert_eq!(state.slots, [Some(16), None, None]);
        assert_eq!(state.selected, 2);

        // And it wraps around like filling does.
        state.select(2);
        state.clear_selected();
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn clearing_all_empties_every_slot_and_resets_the_selection() {
        let mut state = State::default();
        state.fill_selected(16);
        state.fill_selected(2);
        state.select(2);

        state.clear_all();

        assert_eq!(state.slots, [None::<i64>; 3]);
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn manual_selection_clamps_to_the_slots() {
        let mut state = State::default();

        state.select(2);
        assert_eq!(state.selected, 2);

        state.select(7);
        assert_eq!(state.selected, 2);
    }

    #[test]
    fn markers_stay_on_the_bar() {
        let mut state = State::default();

        state.set_start(-4);
        state.set_target(500);
        assert_eq!((state.start, state.target), (0, DEFAULT_MAX_POS));

        state.set_start(60);
        state.set_target(60);
        assert_eq!((state.start, state.target), (60, 60));
    }

    #[test]
    fn slots_convert_to_last_hits() {
        let mut state = State::default();
        assert_eq!(state.last_hits(), Ok(LastHits::default()));

        state.fill_selected(16);
        assert_eq!(state.last_hits().unwrap().slots(), [Some(16), None, None]);

        // Slots are public, so the validation has to hold anyway.
        state.slots[1] = Some(99);
        assert_eq!(state.last_hits(), Err(CoreError::UnknownHit(99)));
    }
}
