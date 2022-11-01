{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};
        src = ./backend/maceats-server;

        # Build dependencies
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          cargoLock = ./backend/Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };

        # Build the crate
        backend = craneLib.buildPackage {
          inherit cargoArtifacts src;
          cargoLock = ./backend/Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };
      in
      rec {
        packages.backend = backend;
        packages.default = backend;

        apps.default = flake-utils.lib.mkApp {
          drv = backend;
        };

        dockerContainers.backend = pkgs.dockerTools.buildLayeredImage {
          name = "app";
          contents = [
            pkgs.dockerTools.fakeNss
            packages.backend
          ];
          config = {
            Cmd = [ apps.default.program ];
            ExposedPorts = {
              "8080/tcp" = { };
            };
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };
      });
}
