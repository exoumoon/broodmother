{
    description = "Crackhead engineering, Vol. 1: Clump of wires";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, ... } @ inputs: {
        devShells."x86_64-linux".default = let
            overlays = [ inputs.rust-overlay.overlays.default ];
            pkgs = import nixpkgs {
                system = "x86_64-linux";
                inherit overlays;
            };
            avr = with pkgs.pkgsCross.avr.buildPackages; [
                binutils
                gcc
                avrdude
                ravedude
                simavr
            ];
        in pkgs.mkShell {
            name = "atmega328p_shell";
            buildInputs = avr ++ [
                (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                    extensions = [ "rust-src" ];
                }))
            ];
        };
    };
}
