{
  description = "url shortener in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) { inherit system overlays; };
        craneLib = crane.mkLib pkgs;
        bin = craneLib.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkgs.protobuf ];
        };
      in
      rec {

        packages.default = bin;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            protobuf
            eza
            fd
            (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
            rust-analyzer
            rustPackages.clippy
            rust-script
            rustfmt
            cargo-watch
            sqlx-cli
            buf
            dbmate
          ];
          DATABASE_URL = "postgres://testing:testing@127.0.0.1:5432/testing?sslmode=disable";
          shellHook = ''
            export PATH=$PATH:$HOME/.cargo/bin/
          '';

        };
      }
    );
}
