use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::fmt::Formatter;
use std::fmt::Debug;

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::virtual_dom::VNode;


pub enum Loadable<T> {
    Unloaded,
    Loading(FetchTask),
    Loaded(T),
    Failed(Option<String>)
}

impl <T> Debug for Loadable<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            Loadable::Unloaded => write!(f, "Unloaded"),
            Loadable::Loading(_) => write!(f, "Loading"),
            Loadable::Loaded(t) => write!(f, "Loaded: {:?}", t),
            Loadable::Failed(_) => write!(f, "Failed"),
        }
    }
}

impl <T> Default for Loadable<T> {
    fn default() -> Self {
        Loadable::Unloaded
    }
}

impl <T> Clone for Loadable<T>
    where T: Clone
{
    fn clone(&self) -> Self {
        match self {
            Loadable::Unloaded => Loadable::Unloaded,
            Loadable::Loading(_) => Loadable::Unloaded, // Any loading loadable throws away the FT because it can't be cloned
            Loadable::Loaded(t) => Loadable::Loaded(t.clone()),
            Loadable::Failed(o) => Loadable::Failed(o.clone())
        }
    }
}

//#[derive(Clone, Debug)]
//pub enum IconType {
//    Normal,
//    Text
//}


const LOADING_SVG: &'static str = include_str!("../../inlined_assets/LoadingRoll.svg");

struct LoadingIcon{}

impl<U, CTX> Renderable<CTX, U> for LoadingIcon
    where
//        CTX: AsMut<ConsoleService> + 'static,
        CTX: 'static,
        U: Component<CTX>

{
    fn view(&self) -> Html<CTX, U> {
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{LOADING_SVG.to_string()};
//            console.log(div);
            return div;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        vnode
    }
}

impl <T> Loadable<T> {

    /// Uses a 100x100 icons for loading and error.
    /// This should work for medium to large sized elements, but if the view area is less than that, visual bugs will result.
    pub fn default_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
        where
            CTX: 'static,
            U: Component<CTX>
    {

        match self {
            Loadable::Unloaded => html! {
                <>
                </>
            },
            Loadable::Loading(_) => html! {
                <div class="flexbox-center",>
                    {LoadingIcon{}.view()}
                </div>
            },
            Loadable::Loaded(ref t) => render_fn(t),
            Loadable::Failed(error) => {
                if let Some(message) = error {
                    html! {
                        <div class="flexbox-center",>
                            {message}
                        </div>
                    }
                }
                else {
                    html! {
                        <div class="flexbox-center",>
                            {"Request Failed"}
                        </div>
                    }
                }
            }
        }
    }

    /// Uses text for all error and loading fillers.
    /// This should allow it to be used most flexibly.
    pub fn small_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
        where
            CTX: 'static,
            U: Component<CTX>
    {

        match self {
            Loadable::Unloaded => html! {
                <>
                </>
            },
            Loadable::Loading(_) => html! {
                <div class="flexbox-center",>
                    {"Loading..."}
                </div>
            },
            Loadable::Loaded(ref t) => render_fn(t),
            Loadable::Failed(error) => {
                if let Some(message) = error {
                    html! {
                        <div class="flexbox-center",>
                            {message}
                        </div>
                    }
                }
                else {
                    html! {
                        <div class="flexbox-center",>
                            {"Request Failed"}
                        </div>
                    }
                }
            }
        }
    }

}