{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";

  outputs = { self, nixpkgs }:
    let
      systems = {
        "x86_64-linux" = "https://github.com/one-d-wide/random-rs/releases/download/v0.2.4/random-rs-linux-x86_64?10bb7d41b241fba516acee4fbfedca23b28e0c7d49c26c5d3389cc317919341e";
        "aarch64-linux" = "https://github.com/one-d-wide/random-rs/releases/download/v0.2.4/random-rs-linux-aarch64?f1c91bdc24270978fc6c382fa4de6d6f6fe50377f2b4b7d1c15d3e0f707a686b";
      };
      lib = nixpkgs.lib;
    in
    {
      packages =
        lib.genAttrs (lib.attrNames systems)
          (system:
            let
              pkgs = nixpkgs.legacyPackages.${system};
              pair = lib.splitString "?" systems.${system};
            in
            rec {
              default = random-rs;
              random = random-rs;
              random-rs = pkgs.stdenv.mkDerivation {
                name = "random-rs";
                src = pkgs.fetchurl { url = lib.head pair; outputHashAlgo = "sha256"; outputHash = lib.tail pair; };
                phases = [ "installPhase" ];
                installPhase = ''
                  mkdir -p $out/bin
                  install -T $src $out/bin/random-rs
                  ln -s random-rs $out/bin/random
                '';
              };

            });

    };
}

