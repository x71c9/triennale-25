{
  description = "Dev environment with Python + Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        dynamixel_sdk = pkgs.callPackage ./dynamixel_sdk.nix {};
        myPython = pkgs.python311.withPackages (ps: with ps; [
          dynamixel_sdk
          pyserial
        ]);
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            myPython
            pkgs.rustc
            pkgs.cargo
          ];

          shellHook = ''
            echo "Shell with Python + Rust ready"
          '';
        };
      }
    );
}

