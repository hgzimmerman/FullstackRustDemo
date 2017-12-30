use msg::Msg;
use yew::html::Html;

pub mod navbar;
pub mod content;
pub mod news;
pub mod bucket_questions;
pub mod loadable;


pub trait Viewable<MSG> {
    fn view(&self) -> Html<MSG>;
}