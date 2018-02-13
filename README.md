A novelty website written entirely in Rust.

The backend uses Rocket (WebServer) + Diesel (ORM).
The frontend uses Yew (React-like framework).

External dependency management is handled with Nix/Nixos.

# Planned Features
W@J intends to support the following features: 
* User accounts.
* "News" article viewing and authoring via markdown.
* A forum system for talking about Joe.
* A Bucket Questions game implementation.

# Build Instructions

* Frontend (Currently not set up for development)
  * Install nightly rust via `rustup`
  * Install the `wasm32-unknown-unknown` target via rustup.
    * Currently wasm32 doesn't work with Yew, so `asmjs-unknown-emscripten` should also be installed.
  * Install `cargo-web` via `cargo install cargo-web`.
  * From the `/www/` directory, run `cargo web build --release --target-webasm` to build the frontend.
    * If targeting asmjs, just run `cargo web build` or `cargo web build --release` to build the frontend instead.
* Backend
  * Install nightly rust via `rustup`.
  * Nixos/Nix is used as the primary configuration management tool. This is an option for Linux and macOS developers.
    * Install nix: `curl https://nixos.org/nix/install | sh`.
    * You are welcome to create a dockerfile with the database, packages and environment variables already set up, independent from the Nix ecosystem, but Nixos/Nix will remain as the primairily supported config tool.
  * Run `nix-shell` from the project root.
  * Postgres will need to be setup to correspond to the DATABASE_URL specified in `default.nix`. (proper instructions forthcoming)
  * From the project root, run `cargo run` to build and run the webserver.
  * For ease of development, run `cargo install cargo-watch` and run `cargo watch -x check` to run rust's type checker whenever you save a file.
  * To build documentation, nix-shell should have set up an alias: `docs` that builds the documentation and opens a tab in the browser for viewing.
