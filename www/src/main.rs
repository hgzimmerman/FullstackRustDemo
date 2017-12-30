#![recursion_limit="128"]
#![feature(vec_remove_item)]


extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate strum;
//#[macro_use]
//extern crate strum_macros;



#[macro_use]
extern crate yew;

//use strum::IntoEnumIterator;
use yew::html::*;

use views::*;

mod views;
mod controller;
mod models;

use controller::{Msg, update};
use models::{Model, Page, NewsModel, Article};
use views::loadable::Loadable;




fn main() {
    let model = Model {
        page: Page::News(NewsModel{
            link_id: "article1".to_string() , // Needed???
            article: Loadable::Loaded(Article::temp())
        })
    };
    program(model, update, views::view);
}
