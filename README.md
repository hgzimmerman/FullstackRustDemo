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
* Initial setup : https://github.com/hgzimmerman/WeekendAtJoes4/wiki/Initial-setup
* Release instructions: 
