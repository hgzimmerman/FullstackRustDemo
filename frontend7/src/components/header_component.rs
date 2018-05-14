use yew::prelude::*;
// use button::Button;
use link::Link;
use context::Context;

use Route;

#[derive(Clone, PartialEq)]
pub struct HeaderLink {
    pub link: Route,
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct Header {
    pub links: Vec<HeaderLink>,
//    pub callback: Option<Callback<Route>>
}

pub enum Msg {
    CallLink(Route),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub links: Vec<HeaderLink>,
//    pub callback: Option<Callback<Route>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            links: vec![],
//            callback: None
        }
    }
}


impl Component<Context> for Header {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Header {
            links: props.links,
//            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::CallLink(route) => {
                //                if let Some(ref cb) = self.callback {
                //                    cb.emit(page_view)
                //                }
                context.routing.set_route(route);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.links = props.links;
        true
    }
}

impl Renderable<Context, Header> for Header {
    fn view(&self) -> Html<Context, Self> {

        let link = |x: &HeaderLink| {
                html! {
                <Link<Route>: name=&x.name, cb_value=&x.link, callback=|pv| Msg::CallLink(pv), classes="nav-link", />
            }
        };

        html! {
            <div class="header",>
                <div class="nav-title",>
                    { "WeekendAtJoe's.com" }
                </div>
                <div class="nav-links",>
                    { for self.links.iter().map(link)}
                </div>
            </div>
        }
    }
}
