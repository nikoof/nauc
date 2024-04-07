{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, crane, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.lib.${system};
        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "naucit-deps";
          buildInputs = with pkgs; [];
          nativeBuildInputs = with pkgs; [];
        });
        naucit = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          propagatedBuildInputs = with pkgs; [
            nasm
          ];
        });
      in {
        checks = {
          crate = naucit;
        };

        packages.default = naucit;

        apps.default = utils.lib.mkApp {
          drv = naucit;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = with pkgs; [
            rust-analyzer
          ];
        };
      }
  );
}
