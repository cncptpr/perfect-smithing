# perfect-smithing — Web interface implementation plan

Executes `WEB_PLAN.md` (repo: `~/Projects/work-playground/perfect-smithing`). This document is
self-contained: every decision is already made. If anything here contradicts `WEB_PLAN.md`,
`WEB_PLAN.md` wins.

Settled interpretations (from the author):

- Bar bound: **180** (matches the CLI default `max_pos`), i.e. 181 unit cells `0..=180`.
- The bar is built from small rectangular boxes (cells), a bit taller than wide.
  "Every 10th larger, every 5th discolored" refers to these cells (0-based): cell `i % 5 == 0`
  slightly discolored, `i % 10 == 0` additionally a little taller, numbers above at every 20.
- Slot row reads **last, second, third**; the solution line shows the forced suffix in
  **execution order** (`... -> third -> second -> last`), per the WEB_PLAN example.
- Solution prefix runs display as `[16, 16, 16] x3` (values repeated + count).
- Minecraft theme with a bundled OFL pixel font (**Monocraft**), no CDN.

## Key decisions

- **One Cargo package** (no workspace, no extra crates): `perfect-smithing` = lib + 2 bins.
- **Leptos 0.8** + **cargo-leptos** (nixpkgs), all solving on the client (wasm). The axum
  server only serves the static site + SSR shell. No `leptos_router` — URL sync is a small
  pure module + History API.
- Toolchain from **devenv** for all development; `flake.nix` for packaging + NixOS module.
- Pin `wasm-bindgen = "=0.2.127"` — must match nixpkgs `wasm-bindgen-cli` (0.2.127 in both
  `devenv.lock`'s pinned nixpkgs and current nixpkgs-unstable). Comment the pin in Cargo.toml.
- Flake input: `github:NixOS/nixpkgs/nixos-unstable` (provides cargo-leptos 0.3.9 w/
  `no_downloads`, wasm-bindgen-cli 0.2.127, binaryen 132, rustc with wasm32 std — all verified).
- Web hit set is fixed: `DEFAULT_STEPS = [-15, -6, -5, -3, 2, 7, 13, 16]`.

## Package layout

```
perfect-smithing/
├── Cargo.toml            # lib + 2 bins + features + [package.metadata.leptos]
├── src/
│   ├── lib.rs            # module declarations; #[cfg(feature="hydrate")] pub fn hydrate()
│   ├── num.rs            # (moved as-is)
│   ├── matrix.rs         # Matrix<T> (from main.rs)
│   ├── apsp.rs           # APSPResult, floyd_warshall
│   ├── solve.rs          # NEW: LastHits, Solution, solve()
│   ├── cache.rs          # load/store cache incl. steps validation
│   ├── state.rs          # pure app state + slot logic (no leptos)
│   ├── url.rs            # pure State <-> query string (no leptos)
│   ├── colors.rs         # hit value -> CSS class (pure)
│   ├── web_solve.rs      # [cfg hydrate|ssr] OnceLock matrix + solution Memo
│   ├── app.rs            # [cfg hydrate|ssr] App + shell()
│   ├── components/       # [cfg hydrate|ssr] slots, buttons, bar, arrows, solution, numbers
│   ├── main.rs           # bin: CLI
│   └── server.rs         # bin: axum server (required-features = ["ssr"])
├── style/main.css        # Minecraft theme
├── public/               # assets-dir (copied to site root)
│   ├── Monocraft.ttf     # committed, OFL
│   ├── OFL.txt
│   └── favicon.svg
├── devenv.nix            # + cargo-leptos, wasm-bindgen-cli, binaryen, processes.web
├── flake.nix / flake.lock
└── WEB_PLAN.md
```

Cargo.toml shape (versions confirmed available):

```toml
[package]
name = "perfect-smithing"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[[bin]]
name = "perfect-smithing"
path = "src/main.rs"
required-features = ["cli"]

[[bin]]
name = "perfect-smithing-web"
path = "src/server.rs"
required-features = ["ssr"]

[features]
default = ["cli"]
cli = ["dep:clap"]
ssr = ["dep:axum", "dep:tokio", "dep:leptos", "dep:leptos_axum", "dep:leptos_meta"]
hydrate = ["dep:leptos", "dep:leptos_meta", "dep:console_error_panic_hook",
           "dep:wasm-bindgen", "dep:web-sys"]

[dependencies]
clap = { version = "4.6", features = ["derive"], optional = true }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
leptos = { version = "0.8", optional = true }
leptos_meta = { version = "0.8", optional = true }
leptos_axum = { version = "0.8", optional = true }
axum = { version = "0.8", optional = true }
tokio = { version = "1", features = ["rt-multi-thread", "net", "macros"], optional = true }
console_error_panic_hook = { version = "0.1", optional = true }
# MUST match nixpkgs wasm-bindgen-cli (0.2.127); wasm-bindgen errors on patch mismatch.
wasm-bindgen = { version = "=0.2.127", optional = true }
web-sys = { version = "0.3", features = ["Window", "Location", "History"], optional = true }

[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"

[package.metadata.leptos]
output-name = "perfect_smithing"
bin-target = "perfect-smithing-web"      # package has 2 bins -> required
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "style/main.css"
assets-dir = "public"
site-addr = "127.0.0.1:3030"
reload-port = 3031
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
lib-profile-release = "wasm-release"
```

Notes:

- `cargo build` / `cargo test` → default features (`cli`): core + state + url + colors only,
  no leptos compiled. The server bin is skipped (`required-features`). cargo-leptos always
  builds with `--no-default-features` + `hydrate`/`ssr`.
- `hydrate()` lives in `lib.rs` behind `#[cfg(feature = "hydrate")]` with
  `#[wasm_bindgen::prelude::wasm_bindgen]` (start-axum template pattern).
- cargo-leptos **generates index.html** — do not create one.
- Event handlers use `leptos::ev::*` types; direct `web_sys` use only in hydrate-gated code
  (window/history/location).
- Reference for server/shell: `https://github.com/leptos-rs/start-axum` (`src/main.rs`,
  `src/app.rs` shell fn, `src/lib.rs`).

---

## Phase 1 — devenv (first; everything else runs inside it)

All cargo/clippy/fmt/wasm commands in every later phase run inside `devenv shell`
(or a direnv-loaded shell, or `devenv up`). Never use a bare-shell toolchain.

- Edit `devenv.nix` (keep existing `languages.rust.enable = true`):

  ```nix
  packages = with pkgs; [ cargo-leptos wasm-bindgen-cli binaryen ];
  processes.web.exec = "cargo leptos watch";
  ```

- Verify (inside `devenv shell`):
  - `cargo --version && rustc --version` (already provided by `languages.rust`)
  - `cargo leptos --version` (0.3.8+), `wasm-bindgen --version` (0.2.127), `wasm-opt --version`
  - `rustc --print target-libdir --target wasm32-unknown-unknown` exists
  - `cargo test && cargo run --quiet -- 60` still works (baseline before touching code)

## Phase 2 — restructure to lib + 2 bins (no behavior change)

- Convert `Cargo.toml` per the sketch above minus leptos deps/metadata (add `cli` feature,
  `[lib]`, CLI bin). Move code out of `main.rs` into modules: `num.rs` (from `src/num.rs`),
  `matrix.rs`, `apsp.rs`, `cache.rs` — pure moves, `solve.rs`/`state.rs`/`url.rs`/`colors.rs`
  come in Phase 3/5. `lib.rs` re-exports what the CLI needs.
- CLI behavior unchanged.

**Verify:** in `devenv shell`: `cargo test`, `cargo clippy --all-targets -- -D warnings`,
`cargo fmt --check`, `cargo run --quiet -- 60` → `It takes 5 steps to get to 60: [16, 16, 13, 13, 2]`.

## Phase 3 — core additions + CLI

`solve.rs`:

```rust
pub const DEFAULT_MAX_POS: i64 = 180;
pub const DEFAULT_STEPS: [i64; 8] = [-15, -6, -5, -3, 2, 7, 13, 16];

pub struct LastHits([Option<i64>; 3]);              // order [last, second, third]
impl LastHits {
    pub fn try_new(values: &[i64], steps: &[i64]) -> Result<Self, CoreError>; // >3 hits, hit ∉ steps
    pub fn execution_values(&self) -> Vec<i64>;      // [third?, second?, last?], empties skipped
}

pub struct Solution {
    pub path: Vec<i64>,                              // positions start..=target
    pub prefix_steps: Vec<i64>,                      // execution order, before suffix
    pub suffix: Vec<(Slot, i64)>,                    // (Third|Second|Last, value), execution order
}

pub fn solve(apsp: &APSPResult, start: i64, target: i64, last: &LastHits)
    -> Result<Solution, CoreError>;                  // Unreachable | SuffixOutOfBounds
```

Algorithm: `pre = target - sum(execution suffix)`; verify the suffix walk stays within
`0..=max_pos`; shortest `start -> pre` via the predecessor matrix; append the suffix walk.
Provably optimal for a fixed suffix.

Cache: add `steps: Vec<i64>` to `APSPResult` (set by `floyd_warshall`). `load_cache(size,
&steps)` returns `None` unless sizes **and** steps match; keep the check as a pure
`validate()` fn (testable) with thin file IO. The existing `cache/result-181.json` lacks the
field → parse fails → recompute + overwrite (desired).

CLI (`src/main.rs`):

- New arg `--last-hits` (comma list, order **last, second, third**, 0–3 values), validated
  against `--steps`; clear error + exit 1 on invalid value/count or when no solution exists.
- Output unchanged; when last-hits are set, append a line:
  `Finishing with: last=16, second=13, third=7`

**Verify:** in `devenv shell`:

- `cargo test` — solve: no last-hits / with last-hits / hit ∉ steps / >3 hits /
  suffix-out-of-bounds / unreachable; cache: steps mismatch, old-format JSON rejected.
- `cargo run --quiet -- 60` unchanged; `cargo run --quiet -- 60 --last-hits 16,2,13` prints
  the extra line; `--last-hits 99,2,2` errors; delete `cache/` → recompute, then
  `--steps 1,2` with old cache → recompute (steps mismatch).

## Phase 4 — leptos scaffold (empty app, end-to-end build)

- Add leptos deps, `ssr`/`hydrate` features, `[package.metadata.leptos]`, `src/lib.rs`
  `hydrate()`, `src/server.rs` (start-axum template main), `src/app.rs` with `shell()`
  + placeholder `App`, `style/main.css` stub, `public/` dir, `[profile.wasm-release]`.
- `server.rs` has `required-features = ["ssr"]`, so no `#[cfg]` stub main is needed.

**Verify:** in `devenv shell`: `cargo build` (CLI-only path still compiles, no leptos),
`cargo leptos build --release` (wasm + server), `cargo leptos serve` →
`curl http://127.0.0.1:3030/` returns the shell; `cargo test` still green.

## Phase 5 — state, URL, solving (headlessly testable)

- `state.rs` — pure:

  ```rust
  pub struct State {
      pub start: i64,                // green player marker, default 0
      pub target: i64,               // red marker, default 0
      pub slots: [Option<i64>; 3],   // [last, second, third]
      pub selected: usize,           // 0..3, default 0 (slot "last")
  }
  // pure fns: fill_selected(value), clear_selected(), select(i) -> with auto-advance
  // fill: slots[selected] = Some(v); selected = (selected + 1) % 3 (wraps)
  // execution suffix = [third?, second?, last?]
  ```

- `url.rs` — pure `to_query(&State) -> String` (omits defaults) and
  `from_query(&str) -> State` (clamp markers to `0..=180`, drop slot values ∉
  `DEFAULT_STEPS`, clamp `sel` to 0..=2). Format:

  `?start=0&target=60&last=16&second=13&third=7&sel=1`

  Only non-default params are written. Hydrate-side plumbing (in `app.rs`, hydrate-gated):
  initial parse from `location.search` before signals are created; a
  `history.replace_state` effect on every change (normalizes + keeps URL shareable, no
  history spam); `popstate` listener re-parses. SSR renders defaults.
- `colors.rs` — `fn class_for(value: i64) -> &'static str`.
- `web_solve.rs` — `thread_local! MATRIX: OnceLock<APSPResult>` built once from
  `DEFAULT_STEPS`/`DEFAULT_MAX_POS` on first access; `Memo<Result<Solution, CoreError>>`
  over `(start, target, slots)` recomputing on every state change. No cache in the web.

**Verify:** `cargo test` — URL round-trip (incl. junk/invalid params), slot fill/advance
wrap, colors cover all 8 values. `cargo clippy --all-targets -- -D warnings`.

## Phase 6 — components (top to bottom, per WEB_PLAN)

1. **Slots** — three slots labeled `last`, `second`, `third` (left→right), value in its hit
   color, empty state visible; selected slot has a Minecraft-style outline; click selects.
2. **Buttons** — 8 hit buttons in game order `[-15,-6,-5,-3,2,7,13,16]` in their hit
   colors + an **empty** button (clears the selected slot, selection stays). Clicking a hit
   button fills the selected slot and advances selection (wrapping).
3. **Numbers** — `0, 20, …, 180`, positioned at the centers of cells 0,20,…,180
   (`left = (v + 0.5) / 181 * 100%`, translateX(-50%)).
4. **Bar** — 181 flex cells (bottom-aligned; base height, `i%10==0` taller, `i%5==0`
   discolored). Markers absolutely positioned at cell centers: **red = target,
   green = player/start**, `pointer-events: none` (cells too) so events always target the
   bar container and `offset_x` maps cleanly: `pos = clamp(floor(offset_x / width * 181), 0, 180)`.
   `click` → target; `contextmenu` → preventDefault + start.
5. **Arrows** — one SVG strip under the bar: each solution hit is a horizontal arrow from
   `p` to `p + step` in its hit color, head pointing left for negative steps. Sequential
   hits are adjacent segments; greedy lane packing (first lane without x-overlap) keeps
   backtracking paths readable.
6. **Solution line** — prefix grouped into maximal consecutive runs in execution order:
   `[16, 16, 16] x3 -> [2] x1`, then filled suffix hits in execution order with values:
   ` -> [7] third -> [13] second -> [16] last`. Everything colored per hit type; omit empty
   parts; both empty → `(no hits needed)`; `Err` → styled `No path found`. Recalculates via
   the Memo on every change.

**Verify:** `cargo leptos build --release && cargo leptos serve`, then smoke-test the
interactions; `cargo test` unchanged.

## Phase 7 — Minecraft theme

- `style/main.css`: dark gray panels with light/dark 3D bevels, no border-radius, chunky
  pixel buttons (hover/active states), `image-rendering: pixelated` where useful.
- `@font-face` Monocraft → `/Monocraft.ttf` (absolute path; the font file is copied by
  `assets-dir` to the site root). Commit `public/Monocraft.ttf` + `public/OFL.txt`
  (download once from github.com/IdreesInc/Monocraft releases; no fetch at build time).
- `public/favicon.svg`: small hand-written pixel-style SVG.
- Hit-color CSS variables: `-15` dark red, `-6` red, `-5` orange, `-3` yellow, `2` green,
  `7` teal, `13` blue, `16` purple (negatives warm, positives cool). Marker colors are
  reserved bright `#ff3030` / `#30ff30` and used nowhere else.
- Page title via `leptos_meta` `<Title/>`.

## Phase 8 — browser verification

Run `devenv up` (process `web` = `cargo leptos watch`) and check in the browser via
`$ agent-browser`:

- Slot auto-advance + wrap, manual slot selection, empty button; selection starts at `last`.
- Bar: left-click moves red target, right-click moves green start; markers snap to cells;
  number/tick rules (20 / 10 / 5) visible.
- Solution + arrows recolor/recalculate on every change; formats match Phase 6 exactly.
- URL: reflects all state, survives reload, works pasted into a new tab, `popstate` works.
- Unreachable input shows the styled error, no panic.
- Final gate: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`
  — all inside `devenv shell`.

## Phase 9 — flake.nix (package + NixOS module)

Inputs: `nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"` (lock pins it). Outputs:
`packages.<sys>.default`, `packages.<sys>.perfect-smithing` (same derivation), `nixosModules.default`
(+ named alias `nixosModules.perfect-smithing`).

Package via `rustPlatform.buildRustPackage`:

```nix
cargoLock.lockFile = ./Cargo.lock;
nativeBuildInputs = [ cargo-leptos wasm-bindgen-cli binaryen makeWrapper ];
buildPhase = ''
  runHook preBuild
  cargo leptos build --release
  cargo build --release --offline --bin perfect-smithing
  runHook postBuild
'';
installPhase = ''
  runHook preInstall
  mkdir -p $out/bin $out/share/perfect-smithing
  cp target/release/perfect-smithing target/release/perfect-smithing-web $out/bin/
  cp -r target/site $out/share/perfect-smithing/site
  wrapProgram $out/bin/perfect-smithing-web --set LEPTOS_SITE_ROOT $out/share/perfect-smithing/site
  runHook postInstall
'';
# default checkPhase (cargo test --offline) stays on
```

(vendored/offline cargo comes from `cargoLock` automatically; `no_downloads` cargo-leptos
uses the PATH tools we provide; `leptos_config::get_configuration(None)` reads
`LEPTOS_SITE_ROOT`/`LEPTOS_SITE_ADDR` at **runtime**, so the wrap + module env work.)

NixOS module options: `enable`, `package` (default `self.packages.${pkgs.system}.default`),
`user`, `group` (defaults `perfect-smithing`; module creates the system user/group),
`address` (default `127.0.0.1`), `port` (default 3000), `openFirewall` (default false),
`extraEnvironment` (attrsOf str). Service: `wantedBy = [ "multi-user.target" ]`,
`serviceConfig = { User, Group, ExecStart = "${package}/bin/perfect-smithing-web",
Restart = "on-failure" }`, `environment = { LEPTOS_SITE_ADDR = "${address}:${port}";
LEPTOS_ENV = "PROD"; } // extraEnvironment`.

**Verify:**

- `nix build .#` → `$out/bin/perfect-smithing` + `$out/bin/perfect-smithing-web` + share/site exist;
  smoke-test: run the wrapped server on a port, `curl` it, kill it.
- Module: evaluate through a throwaway `nixpkgs.lib.nixosSystem` (module + one service
  enablement) via `nix eval --expr` to confirm options/service config; confirm
  `wasm-bindgen-cli.version == "0.2.127"` still holds for the locked input (if nixpkgs
  moved on, bump the `wasm-bindgen` pin in Cargo.toml to match).
- Commit `flake.lock`.

## Risk register

- cargo-leptos ↔ leptos 0.8 wiring (metadata keys, generated index.html, `shell()` signature)
  is validated early in Phase 4 — if something is off, diff against `leptos-rs/start-axum`.
- `wasm-bindgen` pin/CLI mismatch → Phase 4 build fails fast with an explicit message; fix by
  aligning pin with `nixpkgs.wasm-bindgen-cli.version`.
- If the nix sandbox build of the wasm target trips over inherited `RUSTFLAGS`/`HOME`,
  scope/unset them for the `cargo leptos build` invocation (keep it declarative in buildPhase).
