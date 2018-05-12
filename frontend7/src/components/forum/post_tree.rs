
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
use wire::post::EditPostRequest;
use context::networking::RequestWrapper;
use wire::post::NewPostRequest;
use components::markdown;



//use util::color::Color;

pub struct PostTree {
    post: PostData,
    is_reply_active: bool,
    reply_content: String,
    thread_id: i32,
    ft: Option<FetchTask>,
    edit_instance: Option<String>,
    /// Logged in user, unrelated to the post in question. This is a proxy for if a user is logged in.
    user_id: Option<i32>,
}

pub enum Msg {
    ToggleReplyArea,
    UpdateReplyContent(String),
    PostReply,
    ChildPostReady(PostData),
    ChildPostFailed,
    ToggleEditArea,
    UpdateEditContent(String),
    PostPostEdit,
    PostEditReady(PostData),
    PostEditFailed
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: PostData,
    pub thread_id: i32,
    pub user_id: Option<i32>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: PostData::default(),
            thread_id: -1,
            user_id: None
        }
    }
}

impl Component<Context> for PostTree {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {

        // If the user id isn't passed in, get it from localstorage.
        let user_id = if let Some(id) = props.user_id {
            Some(id)
        } else {
            context.user_id().ok()
        };

        PostTree {
            post: props.post,
            is_reply_active: false,
            reply_content: String::new(),
            thread_id: props.thread_id,
            ft: None,
            edit_instance: None,
            user_id
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
                context.log("Post reply failed");
                true
            },
            Msg::ToggleEditArea => {
                if let Some(_) = self.edit_instance {
                    self.edit_instance = None
                } else {
                    self.edit_instance = Some(self.post.content.clone())
                }
                true
            },
            Msg::UpdateEditContent(update_string) => {
                if let Some(ref mut s) = self.edit_instance {
                    *s = update_string
                }
                true
            },
            Msg::PostPostEdit => {
                if let Some(ref edit_text) = self.edit_instance {
                    let callback = context.send_back(
                        |response: Response<Json<Result<PostResponse, Error>>>| {
                            let (meta, Json(data)) = response.into_parts();
                            println!("META: {:?}, {:?}", meta, data);

                            if meta.status.is_success() {
                                let post_data: PostData = data.unwrap().into();

                                Msg::PostEditReady(post_data)
                            } else {
                                Msg::PostEditFailed
                            }
                        },
                    );
                    if let Ok(user_id) = context.user_id() {
                         let post_edit = EditPostRequest {
                             id: self.post.id,
                             thread_id: self.thread_id,
                             content: edit_text.clone(),
                         };

                        let task = context.make_request(RequestWrapper::UpdatePost(post_edit), callback);
                        self.ft = task.ok() // This will cancel a reply if a reply is active.
                    };
                }
                true
            }
            Msg::PostEditReady(childless_edited_post) => {
                context.log("Edit post succeeded.");
                self.post.merge_childless(childless_edited_post);
                self.edit_instance = None;
                true
            }
            Msg::PostEditFailed => {
                //unimplemented!()
                context.log("Edit post failed.");
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
                <PostTree: post=x, thread_id=self.thread_id, user_id=self.user_id, />
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

        fn edit_area_view(edit_instance: &Option<String>, normal_content: &String) -> Html<Context, PostTree> {
            if let Some(edit_content_string) = edit_instance {
                html!{
                    <div>
                        <AuthorMarkdownToggle: text=edit_content_string, callback=|text| Msg::UpdateEditContent(text), />
                        <Button: title="Edit", onclick=|_| Msg::PostPostEdit, />
                    </div>
                }
            } else {
                html! {
                    <div>
                        {markdown::render_markdown(normal_content)}
                    </div>
                }
            }
        }

        fn edit_button_fn(post_tree: &PostTree) -> Html<Context, PostTree> {
            if let Some(user_id) = post_tree.user_id {
                if user_id == post_tree.post.author.id {
                    return html! {
                        <>
                            <Link<()>: name="edit", callback=|_| Msg::ToggleEditArea, />
                        </>
                    }
                }
            }
            return html! {
                <></>
            }
        }

        fn reply_button_fn(post_tree: &PostTree) -> Html<Context,PostTree> {
            // User is logged in
            if let Some(_) = post_tree.user_id {
                html! {
                    <>
                        <Link<()>: name="reply", callback=|_| Msg::ToggleReplyArea, />
                    </>
                }
            } else {
                html! {
                    <></>
                }
            }
        }

        html! {
            <div class=("post-left-pad"),>
                <div class=("post-card", "flexbox-vert"),>
                    <div class=("post-content"),>
                        { edit_area_view(&self.edit_instance, &self.post.content) }
                    </div>
                    <div class=("post-info","flexbox-horiz"),>
                        <div>
                            {"By "}{&self.post.author.display_name}
                        </div>
                        <div>
                            {reply_button_fn(self)}
                        </div>
                        <div>
                            {edit_button_fn(self)}
                        </div>
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