use std::cell::OnceCell;

use leptos::prelude::*;

use crate::apsp::{APSPResult, floyd_warshall};
use crate::solve::{CoreError, DEFAULT_MAX_POS, DEFAULT_STEPS, Solution, solve};
use crate::state::State;

thread_local! {
    /// The web keeps no cache: the matrix is built once per session, on first use.
    static MATRIX: OnceCell<APSPResult> = const { OnceCell::new() };
}

/// The solution for the current state, recomputed whenever the markers or the
/// slots change.
pub fn solution_memo(state: RwSignal<State>) -> Memo<Result<Solution, CoreError>> {
    Memo::new(move |_| {
        let current = state.get();
        let last = current.last_hits()?;
        with_matrix(|matrix| solve(matrix, current.start, current.target, &last))
    })
}

fn with_matrix<T>(f: impl FnOnce(&APSPResult) -> T) -> T {
    MATRIX.with(|matrix| {
        f(matrix.get_or_init(|| floyd_warshall(DEFAULT_MAX_POS + 1, DEFAULT_STEPS.to_vec())))
    })
}
