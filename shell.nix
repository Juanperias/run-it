{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = with pkgs; [
    rustup
    podman
  ];
  shellHook = ''
    rustup default stable
    cargo install runit --path .
  '';
}
