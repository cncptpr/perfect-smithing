use leptos::{html, prelude::*};

use crate::solve::DEFAULT_MAX_POS;
use crate::state::State;

use super::cell_center;

/// The bar: 151 bottom-aligned cells with the red target and the green
/// player marker on top. Cells and markers ignore the pointer, so events
/// always land on the bar container and its width maps the mouse position
/// onto a cell: `pos = clamp(floor(offset / width * 151), 0, 150)`.
/// Left-clicking moves the target, right-clicking the player marker.
#[component]
pub fn Bar(state: RwSignal<State>) -> impl IntoView {
    let bar = NodeRef::<html::Div>::new();

    view! {
        <div
            class="bar"
            node_ref=bar
            on:click=move |ev| {
                if let Some(width) = bar_width(bar) {
                    state.update(|current| current.set_target(cell_from_offset(ev.offset_x(), width)));
                }
            }
            on:contextmenu=move |ev| {
                ev.prevent_default();
                if let Some(width) = bar_width(bar) {
                    state.update(|current| current.set_start(cell_from_offset(ev.offset_x(), width)));
                }
            }
        >
            {(0..=DEFAULT_MAX_POS)
                .map(|cell| {
                    let mut class = String::from("cell");
                    if cell % 5 == 0 {
                        class.push_str(" cell-fifth");
                    }
                    if cell % 10 == 0 {
                        class.push_str(" cell-tick");
                    }
                    view! { <div class=class/> }
                })
                .collect_view()}
            <div
                class="marker marker-target"
                style=move || marker_style(state.get().target)
            />
            <div
                class="marker marker-start"
                style=move || marker_style(state.get().start)
            />
        </div>
    }
}

/// The rendered width of the bar, `None` while it is unmounted or hidden.
fn bar_width(bar: NodeRef<html::Div>) -> Option<i32> {
    bar.get()
        .map(|element| element.offset_width())
        .filter(|width| *width > 0)
}

/// Maps a mouse offset inside the bar to the cell it covers:
/// `clamp(floor(offset / width * 151), 0, 150)`.
pub(crate) fn cell_from_offset(offset: i32, width: i32) -> i64 {
    let cell = f64::from(offset) / f64::from(width) * (DEFAULT_MAX_POS + 1) as f64;
    (cell.floor() as i64).clamp(0, DEFAULT_MAX_POS)
}

/// Horizontally centers a marker on its cell; the theme gives it size,
/// colour and the pointer transparency.
fn marker_style(position: i64) -> String {
    format!("left:{}%", cell_center(position))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_map_onto_cells() {
        // A bar one pixel per cell wide: every pixel is its own cell.
        assert_eq!(cell_from_offset(0, 151), 0);
        assert_eq!(cell_from_offset(1, 151), 1);
        assert_eq!(cell_from_offset(90, 151), 90);
        assert_eq!(cell_from_offset(150, 151), 150);

        // Fractional cells floor down.
        assert_eq!(cell_from_offset(1, 3), 50);
    }

    #[test]
    fn offsets_stay_on_the_bar() {
        assert_eq!(cell_from_offset(0, 1510), 0);
        assert_eq!(cell_from_offset(-40, 1510), 0);
        assert_eq!(cell_from_offset(1510, 1510), 150);
        assert_eq!(cell_from_offset(4000, 1510), 150);
    }

    #[test]
    fn markers_sit_on_the_center_of_their_cell() {
        // Everything but the position comes from the theme.
        assert_eq!(marker_style(75), "left:50%");
        assert_eq!(marker_style(0), format!("left:{}%", cell_center(0)));
    }
}
