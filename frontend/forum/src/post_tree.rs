use common::fetch::FetchResponse;
use common::fetch::Networking;
use datatypes::post::*;
//use context::Context;
use identifiers::thread::ThreadUuid;
use identifiers::user::UserUuid;
use requests::ForumRequest;
use util::button::Button;
use util::link::Link;
use util::markdown;
use util::markdown::author_markdown_toggle::AuthorMarkdownToggle;
use wire::post::EditPostRequest;
//use context::networking::RequestWrapper;
use wire::post::NewPostRequest;
use wire::post::PostResponse;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};


//use util::color::Color;

pub struct PostTree {
    post: PostData,
    is_reply_active: bool,
    reply_content: String,
    thread_id: ThreadUuid,
    edit_instance: Option<String>,
    /// Logged in user, unrelated to the post in question. This is a proxy for if a user is logged in.
    user_id: Option<UserUuid>,
    networking: Networking,
    link: ComponentLink<PostTree>
}

impl PostTree {
    fn make_a_reply_post(&mut self) {
        if let Some(user_id) = self.user_id {
            self.is_reply_active = false; // TODO, not sure if this is ideal to have here?
            let post_reply =  NewPostRequest {
                author_uuid: user_id,
                thread_uuid: self.thread_id,
                parent_uuid: Some(self.post.uuid),
                content: self.reply_content.clone(),
            };
            self.networking.fetch(
                &ForumRequest::CreatePostResponse(post_reply),
                |r: FetchResponse<PostResponse>| Msg::HandleCreatePostReplyResponse(r.map(PostData::from)),
                &self.link
            );
        } else {
            error!("user id should always be accessible")
        }

    }
    fn send_edit_request(&mut self) {
        if let Some(ref edit_text) = self.edit_instance {
            let post_edit = EditPostRequest {
                uuid: self.post.uuid,
                thread_uuid: self.thread_id,
                content: edit_text.clone(),
            };
            self.networking.fetch(
                &ForumRequest::UpdatePost(post_edit),
                |r: FetchResponse<PostResponse>| Msg::HandleEditPostResponse(r.map(PostData::from)),
                &self.link
            );
        }
    }
}

pub enum Msg {
    ToggleReplyArea,
    UpdateReplyContent(String),
    PostReply,
    HandleCreatePostReplyResponse(FetchResponse<PostData>),
//    ChildPostReady(PostData),
//    ChildPostFailed,
    ToggleEditArea,
    UpdateEditContent(String),
    PostPostEdit,
    HandleEditPostResponse(FetchResponse<PostData>),
//    PostEditReady(PostData),
//    PostEditFailed
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub post: PostData,
    pub thread_uuid: ThreadUuid,
    pub user_uuid: Option<UserUuid>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            post: PostData::default(),
            thread_uuid: ThreadUuid::default(),
            user_uuid: None,
        }
    }
}

impl Component for PostTree {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        // If the user id isn't passed in, get it from localstorage.
        let user_id = if let Some(uuid) = props.user_uuid {
            Some(uuid)
        } else {
            let mut storage_service = StorageService::new(Area::Local);
            ::common::user::user_id(&mut storage_service).ok()
        };

        PostTree {
            post: props.post,
            is_reply_active: false,
            reply_content: String::new(),
            thread_id: props.thread_uuid,
            edit_instance: None,
            user_id,
            networking: Networking::new(&link),
            link
        }
    }

    fn update(&mut self, msg: Self::Message ) -> ShouldRender {
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
                self.make_a_reply_post();
                true
            }
            Msg::HandleCreatePostReplyResponse(response) => {
                match response {
                    FetchResponse::Success(post_data) => {
                        self.post.children.push(post_data);
                        self.reply_content = "".to_string();
                    }
                    FetchResponse::Error(_) => {
                        error!("Couldn't respond to post");
                    }
                    FetchResponse::Started => {
                        info!("Started sending response post");
                    }
                }
                true
            }

            Msg::ToggleEditArea => {
                if self.edit_instance.is_some() {
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
                self.send_edit_request();
                true
            }
            Msg::HandleEditPostResponse(response) => {
                match response {
                    FetchResponse::Success(edited_post) => {
                        self.post.merge_childless(edited_post);
                        self.edit_instance = None;
                    }
                    FetchResponse::Error(e) => error!("Edit post failure: {:?}", e),
                    FetchResponse::Started => { info!("Edit post started.")} // TODO, do something here
                }
                true
            }
            Msg::NoOp => false
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<PostTree> for PostTree {
    fn view(&self) -> Html<Self> {

        let child = |x: &PostData| {
            html! {
                <PostTree: post=x, thread_uuid=self.thread_id, user_uuid=self.user_id, />
            }
        };

        fn reply_area_view(post_tree: &PostTree) -> Html<PostTree> {
            if post_tree.is_reply_active {
                html!{
                    <div>
                        <AuthorMarkdownToggle: text=&post_tree.reply_content, callback= Msg::UpdateReplyContent, />
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

        fn edit_area_view(edit_instance: &Option<String>, normal_content: &str) -> Html<PostTree> {
            if let Some(edit_content_string) = edit_instance {
                html!{
                    <div>
                        <AuthorMarkdownToggle: text=edit_content_string, callback=Msg::UpdateEditContent, />
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

        fn edit_button_fn(post_tree: &PostTree) -> Html<PostTree> {
            if let Some(user_id) = post_tree.user_id {
                if user_id == post_tree.post.author.uuid {
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

        fn reply_button_fn(post_tree: &PostTree) -> Html<PostTree> {
            // User is logged in
            if post_tree.user_id.is_some() {
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