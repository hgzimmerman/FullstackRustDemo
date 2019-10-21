A novelty website/demo written entirely in Rust.

The backend uses ~Rocket~ Warp (WebServer) + Diesel (ORM).
The frontend uses Yew (React/Elm-like framework).

External dependency management is handled with Nix/Nixos.



# Status
* User accounts, authentication, articles, forum system, and bucket questions game are implemented on the backend.
* The implementation of a chat system is being finalized.
* ~Bucket questions will require Websockets, so while a REST API is exposed to work with them, much of that functionality will migrate there once set up.~ Bucket questions just use HTTP.
* BucketQuestions, Forums, Auth, and Some user features are implemented in the frontend.
  * The chat system, articles, user management, and password resets are not yet implemented in the frontend.
  
* **Development is suspended at the moment. This project currently serves as an example on how to use Rust to create an integrated webserver + webapp. Development has slowed due to a lack of interest, time, and the fact that the frontend stack suffers greatly from excessively long compiletimes, which makes it hard to work on.** Note: Yew has improved greatly since this was last worked upon. Compile times are down significantly (compiling in debug is now possible). The dominating factors are now a lack of time and interest.

# Alternatives
https://github.com/saschagrunert/webapp.rs This project also shows off how to make a fullstack rust app in a more succinct and organized manner, and is better positioned as a template - provided you agree with choice of actix for the backend with session tokens for authentication versus the choices of warp and JWTs used here.

# Build Instructions
* Initial setup : https://github.com/hgzimmerman/FullstackRustDemo/wiki/Initial-setup
* Release instructions: https://github.com/hgzimmerman/FullstackRustDemo/wiki/Release-Instructions
