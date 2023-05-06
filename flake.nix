{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";

  outputs = { self, nixpkgs }:
    let
      systems = {
        "x86_64-linux" = "https://github.com/one-d-wide/random-rs/releases/download/v0.2.2/random-rs-linux-x86_64?2021529cc4d4fadcc31cdecce83ecf58281f370c293258d2e1fa476752a0dc39";
        "aarch64-linux" = "https://github.com/one-d-wide/random-rs/releases/download/v0.2.2/random-rs-linux-aarch64?01431934b062e31fb691fbf724fb2bf82675cb6b78f8c94d9ff362202c9f1113";
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

