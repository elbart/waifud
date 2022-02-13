{

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in rec {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [
            pkg-config
            openssl
            sqliteInteractive
            libvirt
          ];
        };

        packages.waifuctl = pkgs.stdenv.mkDerivation {
          src = self.defaultPackage."${system}";
          phases = "installPhase";
          installPhase = ''
            mkdir -p $out/bin
            cp $src/bin/waifuctl $out/bin
          '';
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              openssl
              pkg-config
              sqliteInteractive
              libvirt
              dhall
              dhall-json
              go
              goimports
              gopls
              cdrkit
              jq
              jo
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };

      });

}
