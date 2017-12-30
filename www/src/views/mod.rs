pub mod navbar;
pub mod content;
pub mod news;
pub mod bucket_questions;
pub mod loadable;

use yew::html::Html;
use controller::Msg;
use models::Model;


pub trait Viewable<MSG> {
    fn view(&self) -> Html<MSG>;
}

pub fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            { navbar::view() }
            <div>
                {content::view(model)}
            </div>
        </div>
    }
}