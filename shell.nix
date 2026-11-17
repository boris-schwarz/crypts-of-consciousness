{ pkgs ? import <nixpkgs> { } }:

let
  # miniquad dlopen()s these at runtime, so they must be on LD_LIBRARY_PATH
  runtimeLibs = with pkgs; [
    libGL
    libx11
    libxi
    libxkbcommon
    alsa-lib
  ];
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = runtimeLibs;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
}
