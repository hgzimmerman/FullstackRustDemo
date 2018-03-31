use yew::prelude::*;
// use button::Button;
use link::Link;

use PageView;

#[derive(Clone, PartialEq)]
pub struct HeaderLink {
    pub link: PageView,
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct Header {
    pub links: Vec<HeaderLink>,
    pub callback: Option<Callback<PageView>>
}

pub enum Msg {
    CallLink(PageView)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub links: Vec<HeaderLink>,
    pub callback: Option<Callback<PageView>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            links: vec![],
            callback: None
        }
    }
}


impl<CTX: 'static> Component<CTX> for Header {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Header {
            links: props.links,
            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::CallLink(page_view) => {
                if let Some(ref cb) = self.callback {
                    cb.emit(page_view)
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.links = props.links;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Header> for Header {
    fn view(&self) -> Html<CTX, Self> {

        let link = |x: &HeaderLink| html! {
            <Link<PageView>: name=&x.name, cb_value=&x.link, callback=|pv| Msg::CallLink(pv), classes="nav-link", />
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
