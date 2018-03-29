use yew::prelude::*;
use Context;


#[derive(Clone, PartialEq)]
pub enum ForumPage {
    Forums,
    Threads,
    Thread,
    ThreadCreate,
    PostReply
//    ForumCreate
}

pub struct Forum {
    pub child: ForumPage,
}


pub enum Msg {
    SetChild(ForumPage)
}

#[derive(Clone, PartialEq)]
pub struct Props {
    pub child: ForumPage,
    pub callback: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            child: ForumPage::Forums,
            callback: None
        }
    }
}

impl Component<Context> for Forum {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _:&mut Env<Context, Self>) -> Self {
        Forum {
            child: props.child,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetChild(child) => {
                self.child = child;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        self.child = props.child;

        true
    }
}

impl Renderable<Context, Forum> for Forum {

    fn view(&self) -> Html<Context, Self> {

        let page = || {
            match &self.child {
                &ForumPage::Forums => {
                    html! {
                        <>
                        </>
                    }
                }
                &ForumPage::Threads => {
                    html!{
                        <>
                        </>
                    }

                }
                _ => {
                    html! {
                        <>
                        {"Not implemented"}
                        </>
                    }
                }
            }
        };

        return html! {
            <>
                {page()}
            </>
        }
    }
}
