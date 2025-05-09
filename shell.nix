{ pkgs ? import <nixpkgs> {} }:

let
  dynamixel_sdk = pkgs.callPackage ./dynamixel_sdk.nix {};
  myPython = pkgs.python311.withPackages (ps: with ps; [
    dynamixel_sdk
    ps.pyserial
  ]);
in
pkgs.mkShell {
  buildInputs = [
    myPython
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    pkgs.openssl
    pkgs.systemd
  ];

  shellHook = ''
    echo "Shell with Python + Rust ready (classic nix-shell)"
  '';
}

