
use yew::prelude::*;
use datatypes::post::*;

use components::button::Button;
use components::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use components::link::Link;

use context::Context;
use failure::Error;
use yew::services::fetch::Response;
use yew::services::fetch::FetchTask;
use yew::format::Json;
use wire::post::PostResponse;
use context::networking::RequestWrapper;
use wire::post::NewPostRequest;


//use util::color::Color;

pub struct PostTree {
    post: PostData,
    is_reply_active: bool,
    reply_content: String,
    thread_id: i32,
    ft: Option<FetchTask>
}

pub enum Msg {
    ToggleReplyArea,
    UpdateReplyContent(String),
    PostReply,
    ChildPostReady(PostData),
    ChildPostFailed
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: PostData,
    pub thread_id: i32
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: PostData::default(),
            thread_id: -1
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
            reply_content: String::new(),
            thread_id: props.thread_id,
            ft: None
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match  msg {
            Msg::ToggleReplyArea => {
                self.is_reply_active = !self.is_reply_active;
                true
            }
            Msg::UpdateReplyContent(new_content) => {
                self.reply_content = new_content;
                true
            }
            Msg::PostReply => {
                self.is_reply_active = false;
                let callback = context.send_back(
                    |response: Response<Json<Result<PostResponse, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        println!("META: {:?}, {:?}", meta, data);

                        if meta.status.is_success() {
                            let post_data: PostData = data.unwrap().into();

                            Msg::ChildPostReady(post_data)
                        } else {
                            Msg::ChildPostFailed
                        }
                    },
                );
                if let Ok(user_id) = context.user_id() {
                     let post_reply =  NewPostRequest {
                        author_id: user_id,
                        thread_id: self.thread_id,
                        parent_id: Some(self.post.id),
                        content: self.reply_content.clone(),
                    };

                    let task = context.make_request(RequestWrapper::CreatePostResponse(post_reply), callback);
                    self.ft = task.ok()
                };

                true
            }
            Msg::ChildPostReady(post_data) => {
                self.post.children.push(post_data);
                self.reply_content = "".to_string();
                true
            }
            Msg::ChildPostFailed => {
                // TODO print error
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
                <PostTree: post=x, thread_id=self.thread_id, />
            }
        };

        fn reply_area_view(post_tree: &PostTree) -> Html<Context, PostTree> {
            if post_tree.is_reply_active {
                html!{
                    <div>
                        <AuthorMarkdownToggle: text=&post_tree.reply_content, callback=|text| Msg::UpdateReplyContent(text), />
                        <Button: title="Reply", onclick=|_| Msg::PostReply, />
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
                            <Link<()>: name="reply", callback=|_| Msg::ToggleReplyArea, />
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