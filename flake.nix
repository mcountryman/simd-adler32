{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (builtins) fromTOML readFile;

        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [ rust-overlay.overlays.default ];

        mkRustupToolchainShell =
          toolchain:

          pkgs.mkShell {
            packages = with pkgs; [
              # utilities
              just
              gnuplot
              wasmtime
              cargo-fuzz
              cargo-msrv

              # rust
              (
                rust-bin.fromRustupToolchain {
                  channel = "nightly-2025-10-21";
                  profile = "default";
                  components = [ "rust-src" ];
                }
                // toolchain
              )
            ];
          };
      in
      rec {
        devShells.msrv = devShells.v1_50_0;
        devShells.default = mkRustupToolchainShell {
          components = [
            "rust-src"
            "rust-docs"
            "rust-analyzer"
            "miri"
            "clippy"
          ];

          targets = [
            "wasm32-wasip1"
            "aarch64-unknown-linux-gnu"
            "x86_64-unknown-linux-gnu"
          ];
        };

        devShells.v1_50_0 = mkRustupToolchainShell { channel = "1.50.0"; };
        devShells.v1_59_0 = mkRustupToolchainShell { channel = "1.59.0"; };
        devShells.v1_61_0 = mkRustupToolchainShell { channel = "1.61.0"; };
        devShells.v1_89_0 = mkRustupToolchainShell { channel = "1.89.0"; };
      }
    );
}
