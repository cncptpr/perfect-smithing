use std::fmt::Display;

use crate::{apsp::APSPResult, num::Num};

pub const DEFAULT_MAX_POS: i64 = 150;
pub const DEFAULT_STEPS: [i64; 8] = [-15, -6, -5, -3, 2, 7, 13, 16];

/// Failures of the core logic, shared by CLI and web.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreError {
    /// More than three hits were given for the last hits.
    TooManyHits,
    /// A hit is not part of the configured steps.
    UnknownHit(i64),
    /// There is no path from start to target.
    Unreachable,
    /// The forced last hits leave the bar.
    SuffixOutOfBounds,
}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::TooManyHits => write!(f, "at most three last hits are allowed"),
            CoreError::UnknownHit(hit) => {
                write!(f, "hit {hit} is not part of the configured steps")
            }
            CoreError::Unreachable => write!(f, "no path reaches the target from the start"),
            CoreError::SuffixOutOfBounds => write!(f, "the last hits leave the bar"),
        }
    }
}

/// The slot a forced hit occupies. The bars solution line shows them in
/// execution order, `third`, `second`, `last`, i.e. reversed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Third,
    Second,
    Last,
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Slot::Third => write!(f, "third"),
            Slot::Second => write!(f, "second"),
            Slot::Last => write!(f, "last"),
        }
    }
}

/// The forced finishing hits of a smithing run, in the order the game lists
/// them: `[last, second, third]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LastHits([Option<i64>; 3]);

impl LastHits {
    /// Builds from values in game order (`last`, `second`, `third`), missing
    /// values stay empty. Every value has to be one of `steps`.
    pub fn try_new(values: &[i64], steps: &[i64]) -> Result<Self, CoreError> {
        if values.len() > 3 {
            return Err(CoreError::TooManyHits);
        }

        let mut slots = [None; 3];
        for (slot, value) in slots.iter_mut().zip(values) {
            *slot = Some(*value);
        }
        Self::from_slots(slots, steps)
    }

    /// Builds from filled slots, in game order (`last`, `second`, `third`).
    /// Every filled slot has to hold one of `steps`.
    pub fn from_slots(slots: [Option<i64>; 3], steps: &[i64]) -> Result<Self, CoreError> {
        for hit in slots.iter().flatten() {
            if !steps.contains(hit) {
                return Err(CoreError::UnknownHit(*hit));
            }
        }
        Ok(Self(slots))
    }

    pub fn slots(&self) -> [Option<i64>; 3] {
        self.0
    }

    /// The hits in execution order, empty slots skipped.
    pub fn execution_values(&self) -> Vec<i64> {
        self.suffix().into_iter().map(|(_, value)| value).collect()
    }

    /// The slots in execution order (`third`, `second`, `last`), empty skipped.
    fn suffix(&self) -> Vec<(Slot, i64)> {
        [
            (Slot::Third, self.0[2]),
            (Slot::Second, self.0[1]),
            (Slot::Last, self.0[0]),
        ]
        .into_iter()
        .filter_map(|(slot, value)| value.map(|value| (slot, value)))
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution {
    /// Every position on the bar, `start` ..= `target`.
    pub path: Vec<i64>,
    /// The hits before the forced suffix, in execution order.
    pub prefix_steps: Vec<i64>,
    /// The forced suffix, in execution order (`third`, `second`, `last`).
    pub suffix: Vec<(Slot, i64)>,
}

/// Shortest path from `start` to `target` that ends with the forced `last`
/// hits. The suffix is fixed, so the prefix only has to reach
/// `target - sum(suffix)`, which makes the result optimal for that suffix.
pub fn solve(
    apsp: &APSPResult,
    start: i64,
    target: i64,
    last: &LastHits,
) -> Result<Solution, CoreError> {
    let max_pos = apsp.max_pos();
    if !(0..=max_pos).contains(&start) || !(0..=max_pos).contains(&target) {
        return Err(CoreError::Unreachable);
    }

    let suffix = last.suffix();
    let suffix_sum: i64 = suffix.iter().map(|(_, value)| value).sum();
    let pre = target - suffix_sum;

    // Walk the suffix and make sure it stays on the bar.
    let mut position = pre;
    if !(0..=max_pos).contains(&position) {
        return Err(CoreError::SuffixOutOfBounds);
    }
    let mut suffix_positions = Vec::with_capacity(suffix.len());
    for (_, value) in &suffix {
        position += value;
        if !(0..=max_pos).contains(&position) {
            return Err(CoreError::SuffixOutOfBounds);
        }
        suffix_positions.push(position);
    }

    if matches!(apsp.dist(start, pre), Num::Inf) {
        return Err(CoreError::Unreachable);
    }
    let mut path = apsp.path(start, pre);
    if path.is_empty() {
        return Err(CoreError::Unreachable);
    }

    let prefix_steps = path
        .array_windows::<2>()
        .map(|window| window[1] - window[0])
        .collect();

    // `suffix_positions` starts right after `pre` and ends at `target`.
    path.extend(suffix_positions);

    Ok(Solution {
        path,
        prefix_steps,
        suffix,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apsp::floyd_warshall;

    const STEPS: [i64; 3] = [-3, 2, 5];

    /// A bar of `0..=10` with the steps `[-3, 2, 5]`.
    fn fixture() -> APSPResult {
        floyd_warshall(11, STEPS.to_vec())
    }

    #[test]
    fn solves_without_last_hits() {
        let solution = solve(&fixture(), 0, 10, &LastHits::default()).unwrap();

        assert!(solution.suffix.is_empty());
        assert_eq!(solution.path.first(), Some(&0));
        assert_eq!(solution.path.last(), Some(&10));
        assert_eq!(solution.prefix_steps.len(), solution.path.len() - 1);
        assert_eq!(solution.prefix_steps.iter().sum::<i64>(), 10);
        // Two hits of 5 are the shortest way to 10.
        assert_eq!(solution.path.len() - 1, 2);
    }

    #[test]
    fn solves_with_last_hits() {
        // last = 2, second = 5, third = -3
        let last = LastHits::try_new(&[2, 5, -3], &STEPS).unwrap();

        let solution = solve(&fixture(), 0, 10, &last).unwrap();

        assert_eq!(
            solution.suffix,
            vec![(Slot::Third, -3), (Slot::Second, 5), (Slot::Last, 2)]
        );
        // The suffix runs 6 -> 3 -> 8 -> 10, the prefix takes the shortest
        // route 0 -> 2 -> 4 -> 6 to the entry point of the suffix.
        assert_eq!(solution.path, vec![0, 2, 4, 6, 3, 8, 10]);
        assert_eq!(solution.prefix_steps, vec![2, 2, 2]);
    }

    #[test]
    fn execution_order_is_third_second_last() {
        let last = LastHits::try_new(&[16, 2, 13], &DEFAULT_STEPS).unwrap();

        assert_eq!(last.slots(), [Some(16), Some(2), Some(13)]);
        assert_eq!(last.execution_values(), vec![13, 2, 16]);
    }

    #[test]
    fn allows_no_last_hits() {
        assert_eq!(LastHits::try_new(&[], &STEPS).unwrap(), LastHits::default());
        assert_eq!(
            LastHits::try_new(&[5], &STEPS).unwrap().execution_values(),
            vec![5]
        );
    }

    #[test]
    fn rejects_hit_outside_steps() {
        assert_eq!(
            LastHits::try_new(&[99, 2, 2], &STEPS),
            Err(CoreError::UnknownHit(99))
        );
        assert_eq!(
            LastHits::from_slots([Some(2), None, Some(-4)], &STEPS),
            Err(CoreError::UnknownHit(-4))
        );
    }

    #[test]
    fn rejects_more_than_three_hits() {
        assert_eq!(
            LastHits::try_new(&[2, 2, 2, 2], &STEPS),
            Err(CoreError::TooManyHits)
        );
    }

    #[test]
    fn rejects_suffix_leaving_the_bar() {
        // Entry point would be 2 - 5 = -3.
        let entry = LastHits::try_new(&[5], &STEPS).unwrap();
        assert_eq!(
            solve(&fixture(), 0, 2, &entry),
            Err(CoreError::SuffixOutOfBounds)
        );

        // Entry point 1 is fine, but the first hit drops to -2.
        let walk = LastHits::try_new(&[5, -3], &STEPS).unwrap();
        assert_eq!(
            solve(&fixture(), 0, 3, &walk),
            Err(CoreError::SuffixOutOfBounds)
        );
    }

    #[test]
    fn reports_unreachable_targets() {
        // Only even positions are reachable with the step 2.
        let even = floyd_warshall(11, vec![2]);
        assert_eq!(
            solve(&even, 0, 3, &LastHits::default()),
            Err(CoreError::Unreachable)
        );

        // Outside of the bar entirely.
        assert_eq!(
            solve(&fixture(), 0, 50, &LastHits::default()),
            Err(CoreError::Unreachable)
        );
        assert_eq!(
            solve(&fixture(), -1, 5, &LastHits::default()),
            Err(CoreError::Unreachable)
        );
    }
}
