use yew::html::Html;
use views::Viewable;
use controller::Msg;

use yew::html::Context;

//TODO: consider moving this into models/util
/// Encapsulates an entity that will be loaded after accessing the page.
/// Its state will be displayed while it has not loaded.
#[derive(Clone, Debug)]
pub enum Loadable<T> {
    Unloaded,
    Loading,
    Loaded(T)
}

impl <T> Viewable<Msg> for Loadable<T>
    where T: Viewable<Msg> {

    fn view(&self) -> Html<Msg> {

        match *self {
            Loadable::Unloaded => {
                html!{
                    <div>
                        {"Content not loaded"}
                    </div>
                }
            }
            Loadable::Loading => {
                html!{
                    <div>
                        {"Content loading..."}
                    </div>
                }
            }
            Loadable::Loaded(ref content) => {
                html!{
                    <div>
                        {content.view()}
                    </div>
                }
            }
        }

    }
}

