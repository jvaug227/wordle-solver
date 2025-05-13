{ pkgs ? import <nixpkgs> { } }:
{
	wordle-solver = pkgs.callPackage ./derivative.nix {};
}
