A novelty website written entirely in Rust.

The Joooooooke is that I'm putting an exceptional amount of effort into creating a production-quality webapp just for Joe.

The backend uses Rocket (WebServer) + Diesel (ORM).
The frontend uses Yew (React-like framework).

External dependency management is handled with Nix/Nixos.

# Planned Features
W@J intends to support the following features: 
* User accounts.
* "News" article viewing and authoring via markdown.
* A forum system for talking about Joe.
* A Bucket Questions game implementation.
* Chat system, so you can talk to Joe.

# Status
* User accounts, authentication, articles, forum system, and bucket questions game are implemented on the backend.
* The implementation of a chat system is being finalized.
* ~Bucket questions will require Websockets, so while a REST API is exposed to work with them, much of that functionality will migrate there once set up.~ Bucket questions just use HTTP.
* BucketQuestions, Forums, Auth, and Some user features are implemented in the frontend.
  * The chat system, articles, user management, and password resets are not yet implemented in the frontend.
* An alpha will be released in the near feature.

# Build Instructions
* Frontend (Currently not set up for development)
  * Install nightly rust via `rustup`
  * Install the `wasm32-unknown-unknown` target via rustup.
  * Install `cargo-web` via `cargo install cargo-web`.
  * From the `/frontend/app/` directory, run `cargo web build` to build the frontend.
* Backend
  * Install nightly rust via `rustup`.
  * Nixos/Nix is used as the primary configuration management tool. This is an option for Linux and macOS developers.
    * Install nix: `curl https://nixos.org/nix/install | sh`.
    * You are welcome to create a dockerfile with the database, packages and environment variables already set up, independent from the Nix ecosystem, but Nixos/Nix will remain as the primairily supported config tool.
  * Run `nix-shell` from the project root.
  * Postgres will need to be setup to correspond to the DATABASE_URL specified in `default.nix`. (proper instructions forthcoming)
    * After the database account is set up, diesel will need to be installed and configured. Run `cargo install diesel` to install it and `diesel migration run` to run the migrations that set up the database.
  * From the project root, run `cargo run` to build and run the webserver.
  * For ease of development, run `cargo install cargo-watch` and run `cargo watch -x check` to run rust's type checker whenever you save a file.
  * To build documentation, nix-shell should have set up an alias: `docs` that builds the documentation and opens a tab in the browser for viewing.
