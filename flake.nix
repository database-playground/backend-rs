{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      fenix,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        lib = nixpkgs.lib;
        pkgs = nixpkgs.legacyPackages.${system};
        fenix' = pkgs.callPackage fenix { };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = fenix'.latest.cargo;
          rustc = fenix'.latest.rustc;
        };
        crate = rustPlatform.buildRustPackage {
          name = "dp-backend";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          nativeBuildInputs = [ pkgs.protobuf ];
          buildInputs = lib.optionals pkgs.stdenv.isDarwin (
            with pkgs.darwin.apple_sdk; [ frameworks.SystemConfiguration ]
          );

          useNextest = true;
          # unknown variant `2024`, expected one of `2015`, `2018`, `2021`
          auditable = false;
          strictDeps = true;
        };
      in
      {
        checks = {
          inherit crate;
        };
        packages.default = crate;

        packages.docker = pkgs.dockerTools.buildImage {
          name = "dp-backend";
          tag = "latest";

          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ crate ];
            pathsToLink = [ "/bin" ];
          };

          config = {
            Cmd = [ "/bin/backend" ];
            ExposedPorts = {
              "8080/tcp" = { };
            };
            Env = [ "ADDR=0.0.0.0:8080" ];
          };
        };
      }
    );
}
