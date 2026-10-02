{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  languages.rust.enable = true;

  packages = with pkgs; [
    cargo-leptos
    wasm-bindgen-cli
    binaryen
    # nixpkgs rustc ships without rust-lld, the wasm target links through lld.
    lld
  ];

  # Requires the leptos scaffold (Phase 4) before it actually runs.
  processes.web.exec = "cargo leptos watch";
}
