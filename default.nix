with import (builtins.fetchGit {
  name = "nixpkgs-unstable-2019-12-02";
  url = https://github.com/nixos/nixpkgs/;
  # `git ls-remote https://github.com/nixos/nixpkgs-channels nixpkgs-unstable`
  rev = "f97746ba2726128dcf40134837ffd13b4042e95d";
}) {};

stdenv.mkDerivation {
  name = "ben-or-randomized-consensus";

  buildInputs = [
    pkgs.cargo
  ];
}
