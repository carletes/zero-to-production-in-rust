let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = [
    # The Rust toolchain (see `./nix/rust.nix` for the details).
    rust

    # Niv (https://github.com/nmattia/niv), to keep dependencies up-to-date.
    pkgs.niv

    # `cargo udeps`, to strip unneeded dependencies from `Cargo.toml`.
    pkgs.cargo-udeps

    # The command-line tools for PostgreSQL.
    pkgs.postgresql

    # The same version of `sqlx-cli` to the one of the `sqlx` crate we use in
    # `Cargo.toml`.
    #
    # TODO: Ensure we use the Rust version from `nix/rust.nix` to build this.
    #       See:
    #       * https://blog.roman-gonzalez.ca/post/624685914615070720)
    #       * https://euandre.org/2020/10/05/cargo2nix-dramatically-simpler-rust-in-nix.html
    (pkgs.rustPlatform.buildRustPackage rec {
      pname = "sqlx-cli";
      version = "0.5.1";

      src = pkgs.fetchFromGitHub {
        owner = "launchbadge";
        repo = "sqlx";
        rev = "v${version}";
        sha256 = "02phkrcjszs6gdq0yva9fv9f8c4bda0vp9alml7kr5fj65gns8mh";
      };

      cargoSha256 = "1899jwqvdrsdhncg107k0i3w8l496gz0d73zdj2mxnj2lfmpfq0s";

      doCheck = false;
      cargoBuildFlags = [ "--package sqlx-cli" ];

      # As of version 0.5.1, `sqlx-cli` selects the features of `sqlx` that
      # require OpenSSL at run-time (and `pkg-config` at build-time).
      nativeBuildInputs = [ pkgs.pkg-config ];
      buildInputs = [ pkgs.openssl ];
    })

    # Keep this line if you use bash.
    pkgs.bashInteractive
  ];
}
