
use yew::prelude::*;
use datatypes::post::*;

//use util::color::Color;

pub struct PostTree {
   post: PostData
}

pub enum Msg {
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: PostData
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: PostData::default()
        }
    }
}

impl<CTX: 'static> Component<CTX> for PostTree {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        PostTree {
            post: props.post
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {

        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }
}

impl<CTX: 'static> Renderable<CTX, PostTree> for PostTree {
    fn view(&self) -> Html<CTX, Self> {

        let child = |x: &PostData| {
            html! {
                <PostTree: post=x, />
            }
        };
        html! {
            <div>
                <div>
                    {&self.post.content}
                    <span>
                    {"By"}
                    {&self.post.author.display_name}
                    </span>
                </div>
                { for self.post.children.iter().map(child)}
            </div>
        }
    }
}