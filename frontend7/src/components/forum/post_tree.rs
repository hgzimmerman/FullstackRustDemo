
use yew::prelude::*;
use datatypes::post::*;

use components::button::Button;
use components::markdown::author_markdown_toggle::AuthorMarkdownToggle;

use context::Context;

//use util::color::Color;

pub struct PostTree {
    post: PostData,
    is_reply_active: bool,
    reply_content: String
}

pub enum Msg {
    ToggleReplyArea,
    UpdateReplyContent(String)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: PostData,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: PostData::default()
        }
    }
}

impl Component<Context> for PostTree {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        PostTree {
            post: props.post,
            is_reply_active: false,
            reply_content: String::new()
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match  msg {
            Msg::ToggleReplyArea => {
                self.is_reply_active = !self.is_reply_active;
                true
            }
            Msg::UpdateReplyContent(new_content) => {
                self.reply_content = new_content;
                true
            }
        }

    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, PostTree> for PostTree {
    fn view(&self) -> Html<Context, Self> {

        let child = |x: &PostData| {
            html! {
                <PostTree: post=x, />
            }
        };

        fn reply_area_view(post_tree: &PostTree) -> Html<Context, PostTree> {
            if post_tree.is_reply_active {
                html!{
                    <div>
                        <AuthorMarkdownToggle: text=&post_tree.reply_content, callback=|text| Msg::UpdateReplyContent(text), />
                    </div>
                }
            } else {
                html! {
                    <>
                    </>
                }
            }
        }

        html! {
            <div class=("post-left-pad"),>
                <div class=("post-card", "flexbox-vert"),>
                    <div class=("post-content"),>
                        {&self.post.content}
                    </div>
                    <div class=("post-info","flexbox-horiz"),>
                        <span>
                            {"By "}{&self.post.author.display_name}
                        </span>
                        <span>
                            <Button: title="reply", onclick=|_| Msg::ToggleReplyArea, />

                        </span>
                    </div>
                    {reply_area_view(self)}
                </div>
                <div>
                    { for self.post.children.iter().map(child)}
                </div>
            </div>
        }
    }
}