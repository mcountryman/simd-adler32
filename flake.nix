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
        rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        overlays = [
          rust-overlay.overlays.default
        ];
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            # rust
            rust-bin
            rust-analyzer
          ];
        };
      }
    );

}
