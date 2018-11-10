//! This module is responsible for facilitating interaction with the database.
//! Pools and Connections are defined which allow a pool to be specified at startup, and for routes to request a connection from the pool.
//! The files in this module contain functions that interact with the type specified by the filename.
//!
//! These functions are analogous to stored procedures with the caveat that performance may be lower
//! due to aggregation and organization of types belonging to different tables occurs server-side
//! instead of on the database.

//#![recursion_limit="512"]
//#![recursion_limit="256"]
// #![feature(use_extern_macros)]
#![feature(test)]
// TODO remove this once the use of macros stops emitting a stupid quantity of warnings
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(drain_filter)]
extern crate test;

extern crate auth as auth_lib;
extern crate error;
extern crate wire;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate identifiers;
extern crate pool;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate slug;
extern crate typename;
extern crate uuid;
#[macro_use]
extern crate typename_derive;
#[macro_use(log)]
extern crate log;
extern crate simplelog;

mod calls;
mod diesel_extensions;
pub use crate::calls::*;
mod conversions;
pub mod schema;

pub use crate::{
    answer::Answer,
    article::Article,
    bucket::Bucket,
    chat::Chat,
    forum::{
        Forum,
        NewForum,
    },
    message::Message,
    post::Post,
    question::Question,
    thread::{
        NewThread,
        Thread,
    },
    user::User,
};
