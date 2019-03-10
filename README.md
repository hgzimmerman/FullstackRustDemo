A novelty website/demo written entirely in Rust.

The backend uses Rocket (WebServer) + Diesel (ORM).
The frontend uses Yew (React/Elm-like framework).

External dependency management is handled with Nix/Nixos.

# Planned Features

* User accounts.
* "News" article viewing and authoring via markdown.
* A forum system.
* A Bucket Questions game implementation.
* Chat system.

# Status
* User accounts, authentication, articles, forum system, and bucket questions game are implemented on the backend.
* The implementation of a chat system is being finalized.
* ~Bucket questions will require Websockets, so while a REST API is exposed to work with them, much of that functionality will migrate there once set up.~ Bucket questions just use HTTP.
* BucketQuestions, Forums, Auth, and Some user features are implemented in the frontend.
  * The chat system, articles, user management, and password resets are not yet implemented in the frontend.
  
* Development is mostly suspended at the moment. This project currently serves as an example on how to use Rust to create an integrated webserver + webapp. Development has slowed due to a lack of interest, time, and the fact that this stack suffers greatly from excessively long compiletimes, which makes it hard to work on.

# Build Instructions
* Initial setup : https://github.com/hgzimmerman/FullstackRustDemo/wiki/Initial-setup
* Release instructions: https://github.com/hgzimmerman/FullstackRustDemo/wiki/Release-Instructions
