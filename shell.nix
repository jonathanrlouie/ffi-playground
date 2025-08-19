{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    rustc
    cargo
    rustfmt
    rustPackages.clippy
    rust-bindgen
    rust-cbindgen
    valgrind
  ];
}

