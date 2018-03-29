use yew::prelude::*;
// use button::Button;
use datatypes::minimal_thread::MinimalThread;



#[derive(Clone, PartialEq)]
pub struct HeaderLink {
    pub link: Callback<()>,
    pub name: String,
    pub id: usize

}

#[derive(Clone, PartialEq)]
pub struct Header {
    pub links: Vec<HeaderLink>
}

pub enum Msg {
    CallLink(usize)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    links: Vec<HeaderLink>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            links: vec![]
        }
    }
}


impl<CTX: 'static> Component<CTX> for Header {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Header {
            links: props.links 
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::CallLink(link_number) => {

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
        use link::Link;
        use link;

        let link = |x: &HeaderLink| html! {
            <Link: name=&x.name, callback=|_| Msg::CallLink(1), classes="", />
        };

        html! {
            <div class="header",>
                { "WeekendAtJoes.com" }

                { for self.links.iter().map(link)}
            </div>
        }
    }
}
