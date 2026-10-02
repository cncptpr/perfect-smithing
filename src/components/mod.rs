//! The interface components, top to bottom as the WEB_PLAN lays them out.
//!
//! Every component reads the state signal or the solution memo and emits
//! structure, positioning and hit classes. Colours and the Minecraft look
//! belong to the theme, which styles these classes.

mod arrows;
mod bar;
mod buttons;
mod marker_inputs;
mod marker_values;
mod numbers;
mod slots;
mod solution;

pub use arrows::Arrows;
pub use bar::Bar;
pub use buttons::HitButtons;
pub use marker_inputs::MarkerInputs;
pub use marker_values::MarkerValues;
pub use numbers::Numbers;
pub use slots::Slots;
pub use solution::SolutionLine;

use crate::solve::DEFAULT_MAX_POS;

/// The percentage at which the center of `position`'s cell sits on the bar:
/// `(position + 0.5) / 151 * 100`. Numbers, markers and arrows all line up
/// on this one formula.
pub(crate) fn cell_center(position: i64) -> f64 {
    (position as f64 + 0.5) / (DEFAULT_MAX_POS + 1) as f64 * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_centers_stay_inside_the_bar() {
        assert_eq!(cell_center(0), 0.5 / 151.0 * 100.0);
        assert_eq!(cell_center(75), 50.0);
        assert_eq!(cell_center(150), 150.5 / 151.0 * 100.0);
    }

    #[test]
    fn cell_centers_rise_with_the_position() {
        let centers: Vec<f64> = (0..=DEFAULT_MAX_POS).map(cell_center).collect();

        assert!(centers.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
