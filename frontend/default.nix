with import <nixpkgs> {
  overlays = map (uri: import (fetchTarball uri)) [
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  ];
};

stdenv.mkDerivation {
  name = "rust-wasm";
  buildInputs = [
    cargo-web
    (latest.rustChannels.nightly.rust.override {
      targets = ["wasm32-unknown-unknown"];
    })
  ];
}
