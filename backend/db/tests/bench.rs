#![feature(test)]
extern crate test;

extern crate wire;
extern crate identifiers;
extern crate db;
extern crate auth as auth_lib;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate migrations_internals;




mod calls;
mod common;