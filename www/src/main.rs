#![recursion_limit="128"]
#![feature(vec_remove_item)]


extern crate strum;
//#[macro_use]
//extern crate strum_macros;

#[macro_use]
extern crate yew;

//use strum::IntoEnumIterator;
use yew::html::*;

use views::*;

mod views;
mod msg;
mod models;

use msg::Msg;
use models::{Model, Page, NewsModel, Article};
use views::loadable::Loadable;


fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    use Msg::*;
    match msg {
        SetTopLevelPage(page) => {
            model.page = page;
        }
    }
}

fn main() {
    let model = Model {
        page: Page::News(NewsModel{
            link_id: "article1".to_string() , // Needed???
            article: Loadable::Loaded(Article::temp())
        })
    };
    program(model, update, views::view);
}
