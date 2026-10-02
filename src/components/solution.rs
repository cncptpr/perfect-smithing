use leptos::prelude::*;

use crate::colors::class_for;
use crate::solve::{CoreError, Solution};

/// One span of the solution line: the text and the class colouring it with
/// its hit type.
struct Piece {
    text: String,
    class: &'static str,
}

/// The solution line below the bar. The prefix is grouped into maximal runs
/// of equal hits, the forced suffix follows slot by slot, all in execution
/// order and coloured per hit. Without any hits the line reads
/// `(no hits needed)`, a failed solve reads `No path found`.
#[component]
pub fn SolutionLine(solution: Memo<Result<Solution, CoreError>>) -> impl IntoView {
    view! {
        <div class="solution">
            {move || {
                let pieces = match solution.get() {
                    Ok(current) => solution_pieces(&current),
                    Err(_) => vec![Piece {
                        text: "No path found".to_string(),
                        class: "solution-error",
                    }],
                };
                pieces.into_iter().map(piece_view).collect_view()
            }}
        </div>
    }
}

/// The line as coloured pieces, e.g.
/// `[16, 16, 16] x3 -> [2] x1 -> [7] third -> [13] second -> [16] last`.
fn solution_pieces(solution: &Solution) -> Vec<Piece> {
    let mut content = Vec::new();

    // The prefix as maximal runs of equal hits, in execution order.
    let mut run_start = 0;
    while run_start < solution.prefix_steps.len() {
        let mut run_end = run_start + 1;
        while run_end < solution.prefix_steps.len()
            && solution.prefix_steps[run_end] == solution.prefix_steps[run_start]
        {
            run_end += 1;
        }
        let run = &solution.prefix_steps[run_start..run_end];
        let values = run
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        content.push(Piece {
            text: format!("[{values}] x{}", run.len()),
            class: class_for(run[0]),
        });
        run_start = run_end;
    }

    // The forced suffix, slot by slot, in execution order.
    for &(slot, value) in &solution.suffix {
        content.push(Piece {
            text: format!("[{value}] {slot}"),
            class: class_for(value),
        });
    }

    if content.is_empty() {
        return vec![Piece {
            text: "(no hits needed)".to_string(),
            class: "no-hits",
        }];
    }

    // Join the pieces with the neutral separator.
    let mut pieces = Vec::with_capacity(content.len() * 2 - 1);
    for (index, piece) in content.into_iter().enumerate() {
        if index > 0 {
            pieces.push(Piece {
                text: " -> ".to_string(),
                class: "solution-sep",
            });
        }
        pieces.push(piece);
    }
    pieces
}

/// Renders one piece as a span; the class carries the hit colour.
fn piece_view(piece: Piece) -> impl IntoView {
    view! { <span class=piece.class>{piece.text}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::Slot;

    fn texts(pieces: &[Piece]) -> Vec<(&str, &str)> {
        pieces
            .iter()
            .map(|piece| (piece.text.as_str(), piece.class))
            .collect()
    }

    fn solution(prefix_steps: Vec<i64>, suffix: Vec<(Slot, i64)>) -> Solution {
        Solution {
            path: vec![],
            prefix_steps,
            suffix,
        }
    }

    #[test]
    fn groups_the_prefix_into_maximal_runs() {
        let solution = solution(
            vec![16, 16, 16, 2],
            vec![(Slot::Third, 7), (Slot::Second, 13), (Slot::Last, 16)],
        );

        assert_eq!(
            texts(&solution_pieces(&solution)),
            [
                ("[16, 16, 16] x3", "hit-16"),
                (" -> ", "solution-sep"),
                ("[2] x1", "hit-2"),
                (" -> ", "solution-sep"),
                ("[7] third", "hit-7"),
                (" -> ", "solution-sep"),
                ("[13] second", "hit-13"),
                (" -> ", "solution-sep"),
                ("[16] last", "hit-16"),
            ]
        );
    }

    #[test]
    fn a_suffix_only_line_has_no_leading_separator() {
        let solution = solution(vec![], vec![(Slot::Second, 7), (Slot::Last, 2)]);

        assert_eq!(
            texts(&solution_pieces(&solution)),
            [
                ("[7] second", "hit-7"),
                (" -> ", "solution-sep"),
                ("[2] last", "hit-2"),
            ]
        );
    }

    #[test]
    fn a_prefix_only_line_has_no_trailing_separator() {
        let solution = solution(vec![2, 2], vec![]);

        assert_eq!(texts(&solution_pieces(&solution)), [("[2, 2] x2", "hit-2")]);
    }

    #[test]
    fn runs_only_split_on_a_changed_hit() {
        let solution = solution(vec![2, -3, -3, -3, 2], vec![]);

        assert_eq!(
            texts(&solution_pieces(&solution)),
            [
                ("[2] x1", "hit-2"),
                (" -> ", "solution-sep"),
                ("[-3, -3, -3] x3", "hit-n3"),
                (" -> ", "solution-sep"),
                ("[2] x1", "hit-2"),
            ]
        );
    }

    #[test]
    fn without_any_hits_the_line_says_so() {
        let solution = solution(vec![], vec![]);

        assert_eq!(
            texts(&solution_pieces(&solution)),
            [("(no hits needed)", "no-hits")]
        );
    }
}
