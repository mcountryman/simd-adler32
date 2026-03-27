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
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [
          rust-overlay.overlays.default
        ];

        packages = with pkgs; [
          # utilities
          just
          wasmtime
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = packages ++ [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
        };

        devShells.msrv = pkgs.mkShell {
          packages = packages ++ [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.msrv.toml) ];
        };
      }
    );

}
