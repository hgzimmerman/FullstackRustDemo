use yew::virtual_dom::VNode;
use yew::prelude::*;
use wrappers::empty_vdom_node;
use stdweb::web::Node;
use stdweb::unstable::TryFrom;


pub enum LoadingType<U>
    where
        U: Component
{
    #[allow(dead_code)]
    Empty,
    #[allow(dead_code)]
    Rolling{diameter: usize},
    Fidget{diameter: usize},
    #[allow(dead_code)]
    Custom(fn() -> Html<U>)
}


impl<U> Renderable<U> for LoadingType< U>
where
    U: Component
{
    fn view(&self) -> Html<U> {
        match self {
            LoadingType::Rolling {diameter} => {
                let style = format!("width: {}px; height: {}px;", diameter, diameter);
                html! {
                    <div class="flexbox-center",>
                        <div style=style,>
                            {LoadingIcon(ROLLING_SVG).view()}
                        </div>
                    </div>
                }
            }
            LoadingType::Fidget {diameter} => {
                let style = format!("width: {}px; height: {}px;", diameter, diameter);
                html! {
                    <div class=("flexbox-center", "full-height", "full-width"),>
                        <div style=style,>
                            {LoadingIcon(FIDGET_SVG).view()}
                        </div>
                    </div>
                }
            }
            LoadingType::Empty => {
                empty_vdom_node()
            }
            LoadingType::Custom(render_fn) => {
                render_fn()
            }
        }
    }
}


/// This svg indicates loading
const ROLLING_SVG: &'static str = include_str!("../inlined_assets/LoadingRoll.svg");
const FIDGET_SVG: &'static str = include_str!("../inlined_assets/Fidget.svg");

struct LoadingIcon (&'static str);

impl<U> Renderable<U> for LoadingIcon
    where
        U: Component

{
    fn view(&self) -> Html<U> {
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{self.0.to_string()};
            return div;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        vnode
    }
}