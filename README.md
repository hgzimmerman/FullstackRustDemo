A novelty website written entirely in Rust.

The backend uses Rocket (WebServer) + Diesel (ORM).
The frontend uses Yew (React-like framework).

External dependency management is handled with Nix/Nixos.


# Build Instructions
* Install nightly rust via `rustup`
* Install the `wasm32-unknown-unknown` target via rustup.
* Install `cargo-web` via `cargo install cargo-web`.
* From the `/www/` directory, run `cargo web build --release --target-webasm` to build the frontend.
* Make sure that `libsql`, `sqlite`, and `postgresql` are installed.
  * Alternatively, install the Nix package manager and run `nix-shell` from the project root.
* From the project root, run `cargo run --release` to build and run the webserver.
