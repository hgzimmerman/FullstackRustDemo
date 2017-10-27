{ }:

let
  pkgs = import <nixpkgs> { };
in
  pkgs.stdenv.mkDerivation {
    name = "WeekendAtJoes4";
    src = ./.;
    buildInputs = [pkgs.libmysql pkgs.sqlite pkgs.postgresql];
    installPhase= ''
        cargo build --release
    '';
  }