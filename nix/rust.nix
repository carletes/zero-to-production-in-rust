{ sources ? import ./sources.nix }:
let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2021-03-18";
  # channel = "stable";
  # date = "2021-02-11";  # Release 1.50.0
  targets = [
    # "aarch64-unknown-linux-gnu"
    # "aarch64-unknown-linux-musl"
    # "x86_64-unknown-linux-gnu"
    # "x86_64-unknown-linux-musl"
  ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in
chan.override {
  extensions = [
    # "clippy-preview"
    # "rls-preview"
    # "rustfmt-preview"
    # "rust-analysis"
    # "rust-std"
    "rust-src"
  ];
}
