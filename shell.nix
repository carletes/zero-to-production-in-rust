let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = [
    # The Rust toolchain (see `./nix/rust.nix` for the details).
    rust

    # The command-line tools for PostgreSQL.
    pkgs.postgresql

    # Ideally we would be installing the same version for `sqlx-cli` as the
    # version od `sqlx` we're using in this project.
    pkgs.sqlx-cli

    # Keep this line if you use bash.
    pkgs.bashInteractive
  ];
}
