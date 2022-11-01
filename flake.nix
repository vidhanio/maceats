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
        src = ./backend;

        # Build dependencies
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;

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

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };
      in
      {
        packages.backend = backend;
        packages.default = backend;

        apps.default = flake-utils.lib.mkApp {
          drv = backend;
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
