# Webinterface for this program

## Minigame

This program is used for a smithing minigame.

The player sees a "progress bar" and 8 buttons.

The bar has two markers and goes from 0 to a constant (I think 170 or so). The first marker is the players position, the second one is the target position.

The eight buttons each represent a different hit, and have a value by which the player marker is moved when clicked. There player needs to get to the target, while staying inside the bounds of the bar, aka. not dropping below 0.

Each different item has a different target (may even change accross plays) and up to three hits, which need to be the last ones for the smithing to succeed.

## What already exist

A cli too that you provide with the target, optional steps (the hit values), optional start etc, which will calcualte, cache and print the perfect path.

## Additions to the core logic

A way to define the last hits (validate against steps).
For consistency with the game these hits should at least be visualized in the order 'last', 'second', 'third'.

Store the configured steps in the cache, and ignore cache if the steps do not match.

## Webinterface

I want this program to be usable via a website.
I want all calculations to happen on the client, but I want to stay with rust, so use something like leptos.
If the matrix is kept in memory, caching is not important for the web.

### Elements
I want the ui exists of (top to bottom):
1. Slots for the last three hits.
2. The eight button (for filling the last three hits)
3. The numbers for the bar
4. The bar
5. The solution to the minigame

Re-calculate the solution on the fly, everytime something changes.
Use different colors for each hit type.

The numbers on top of the bar are written in 20-steps.
Every 10th bar is a little larger, every 5th is slightly discolored.
The marker are red (taget) and green (player) on the bar.

Show the hit's a little bit differently then the printed output, e.g.:
[16, 16, 16] x3 -> [2] x1 -> third -> second -> last
Colored of course.

Also have little horizontal arrows under the bar, that visualize the path of the hits.

All selected state should be stored in the url for sharing.

### Interactions
Have the first ('last') hit slot selected.
Each time one of the eight buttons is clicked, the slot is filled and selection automatically shift to the next one, wrapping around. The slots can be clicked on for manual selection. Also have a 'empty' button, to clear one slot.

Left-clicking the bar should move the target marker, right-clicking the player maker/starting point.

### Theme

Make it Minecraft UI themed.
An update to make it look like the actual minigame design will likely come.

## Development

1. Keep it clean. No hacky solutions allowed. No custom script wanted.
2. Everything needs to be reproducable, so declare it in `./devenv.nix` and use `$ devenv`
3. Use `cargo-leptos` package for building and bundeling.
4. Package for nix: At the end, add a `flake.nix`, which can build the program, and exposes a package, and a module declaring a systemd service with a few common nix options exposed (e.g. user, port, ...).

