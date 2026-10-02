use leptos::prelude::*;

use crate::colors::class_for;
use crate::solve::{CoreError, DEFAULT_STEPS, Solution};

use super::cell_center;

/// The vertical space one arrow lane takes, in pixels.
const LANE_HEIGHT: i32 = 12;
/// The stroke width of the arrow shafts, in pixels.
const STROKE_WIDTH: i32 = 2;
/// The width and height of an arrowhead, in pixels.
const HEAD: i32 = 6;

/// The arrows strip under the bar: every hit of the solution becomes a
/// horizontal arrow from its cell to the cell it lands on, in the hit
/// colour. Arrows that would draw over each other only share a lane when
/// their x ranges do not touch, which keeps backtracking paths readable.
#[component]
pub fn Arrows(solution: Memo<Result<Solution, CoreError>>) -> impl IntoView {
    view! {
        {move || {
            let Ok(current) = solution.get() else {
                return None;
            };
            let hits = hits_of(&current);
            if hits.is_empty() {
                return None;
            }

            let lanes = pack_lanes(&hits);
            let used_lanes = lanes.iter().copied().max().unwrap_or(0) + 1;
            let height = used_lanes as i32 * LANE_HEIGHT;

            Some(
                view! {
                    <svg class="arrows" width="100%" height=height>
                        <defs>
                            {DEFAULT_STEPS
                                .into_iter()
                                .map(|step| {
                                    let class = class_for(step);
                                    view! {
                                        <marker
                                            id=format!("arrow-head-{class}")
                                            markerUnits="userSpaceOnUse"
                                            markerWidth=HEAD
                                            markerHeight=HEAD
                                            refX=HEAD
                                            refY=HEAD / 2
                                            orient="auto"
                                        >
                                            <path
                                                class=class
                                                d="M0 0 L6 3 L0 6 Z"
                                                fill="currentColor"
                                            />
                                        </marker>
                                    }
                                })
                                .collect_view()}
                        </defs>
                        {hits
                            .into_iter()
                            .zip(lanes)
                            .map(|((from, to), lane)| {
                                let class = class_for(to - from);
                                let y = LANE_HEIGHT / 2 + lane as i32 * LANE_HEIGHT;
                                view! {
                                    <line
                                        class=format!("arrow {class}")
                                        x1=format!("{:.4}%", cell_center(from))
                                        x2=format!("{:.4}%", cell_center(to))
                                        y1=y
                                        y2=y
                                        stroke="currentColor"
                                        stroke-width=STROKE_WIDTH
                                        marker-end=format!("url(#arrow-head-{class})")
                                    />
                                }
                            })
                            .collect_view()}
                    </svg>
                },
            )
        }}
    }
}

/// The arrow of every hit: from cell to cell, in execution order. Walking
/// the path covers the prefix and the forced suffix alike.
fn hits_of(solution: &Solution) -> Vec<(i64, i64)> {
    solution
        .path
        .windows(2)
        .map(|step| (step[0], step[1]))
        .collect()
}

/// Greedily packs the arrows into lanes: every arrow lands in the first
/// lane whose arrows do not overlap it in x. Arrows may touch end to end,
/// only drawn over each other counts as an overlap.
pub(crate) fn pack_lanes(hits: &[(i64, i64)]) -> Vec<usize> {
    let mut lanes: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut packed = Vec::with_capacity(hits.len());

    for &(from, to) in hits {
        let range = (from.min(to), from.max(to));
        let taken = lanes.iter().position(|lane| {
            lane.iter()
                .all(|other| range.1 <= other.0 || other.1 <= range.0)
        });
        let lane = match taken {
            Some(lane) => lane,
            None => {
                lanes.push(Vec::new());
                lanes.len() - 1
            }
        };
        lanes[lane].push(range);
        packed.push(lane);
    }

    packed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve::Slot;

    #[test]
    fn sequential_hits_walk_one_lane() {
        let solution = Solution {
            path: vec![0, 16, 32],
            prefix_steps: vec![16, 16],
            suffix: vec![],
        };

        assert_eq!(hits_of(&solution), [(0, 16), (16, 32)]);
        assert_eq!(pack_lanes(&hits_of(&solution)), [0, 0]);
    }

    #[test]
    fn backtracking_hits_move_to_another_lane() {
        // 0 -> 16, then the -15 hit draws back over almost the same x range.
        let packed = pack_lanes(&[(0, 16), (16, 1)]);

        assert_eq!(packed, [0, 1]);
    }

    #[test]
    fn the_first_free_lane_wins() {
        let packed = pack_lanes(&[(0, 10), (5, 15), (10, 20)]);

        assert_eq!(packed, [0, 1, 0]);
    }

    #[test]
    fn disjoint_hits_share_a_lane() {
        assert_eq!(pack_lanes(&[(0, 16), (20, 36)]), [0, 0]);
    }

    #[test]
    fn the_forced_suffix_is_walked_too() {
        let solution = Solution {
            path: vec![0, 2, 4, 6, 3, 8, 10],
            prefix_steps: vec![2, 2, 2],
            suffix: vec![(Slot::Third, -3), (Slot::Second, 5), (Slot::Last, 2)],
        };

        assert_eq!(
            hits_of(&solution),
            [(0, 2), (2, 4), (4, 6), (6, 3), (3, 8), (8, 10)]
        );
    }

    #[test]
    fn a_path_without_steps_has_no_arrows() {
        let solution = Solution {
            path: vec![7],
            prefix_steps: vec![],
            suffix: vec![],
        };

        assert!(hits_of(&solution).is_empty());
        assert!(pack_lanes(&hits_of(&solution)).is_empty());
    }
}
