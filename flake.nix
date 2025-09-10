{
  description = "url shortener in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) { inherit system overlays; };
        naersk' = pkgs.callPackage naersk { };
      in
      rec {

        defaultPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = [ pkgs.protobuf ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs;[
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
