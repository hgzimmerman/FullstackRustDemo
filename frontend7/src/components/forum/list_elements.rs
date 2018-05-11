use yew::prelude::*;
use Context;

use datatypes::forum::ForumData;
use datatypes::thread::MinimalThreadData;
use components::link::Link;

use components::forum::ForumModel;

impl Renderable<Context, ForumModel> for ForumData {
   fn view(&self) -> Html<Context, ForumModel> {
        html! {
            <li class="forum-list-element",>
                <div>
                    <Link<ForumData>: name=&self.title, cb_value=self, callback=|forum_data| super::Msg::SetForum{forum_data}, classes="forum-link", />
                </div>
                <div>
                    {&self.description}
                </div>
            </li>
        }
   }
}

impl Renderable<Context, ForumModel> for MinimalThreadData {
   fn view(&self) -> Html<Context, ForumModel> {
        html! {
            <li class="forum-list-element",>
                <div>
                    <Link<i32>: name=&self.title, cb_value=self.id, callback=|id| super::Msg::SetThread {thread_id: id}, classes="forum-link", />
                </div>
                <div>
                    {format!("By: {}", &self.author.display_name)}
                </div>
            </li>
        }
   }
}
