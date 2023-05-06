{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";

  outputs = { self, nixpkgs }:
    let
      systems = {
        "x86_64-linux" = "https://github.com/one-d-wide/random-rs/releases/latest/download/random-rs-linux-x86_64?";
        "aarch64-linux" = "https://github.com/one-d-wide/random-rs/releases/latest/download/random-rs-linux-aarch64?";
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
