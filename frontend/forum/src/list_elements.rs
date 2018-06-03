use yew::prelude::*;
use Context;

use datatypes::forum::ForumData;
use datatypes::thread::MinimalThreadData;
use util::link::Link;

use ForumModel;
use datatypes::thread::SelectableMinimalThreadData;
use identifiers::thread::ThreadUuid;

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

impl Renderable<Context, ForumModel> for SelectableMinimalThreadData {
   fn view(&self) -> Html<Context, ForumModel> {

       fn element_internals(minimal_thread_data: &MinimalThreadData) -> Html<Context,ForumModel> {
           html! {
               <>
                   <div>
                        <Link<ThreadUuid>: name=&minimal_thread_data.title, cb_value=minimal_thread_data.id, callback=|id| super::Msg::SetThread {thread_id: id}, classes="forum-link", />
                   </div>
                   <div>
                        {format!("By: {}", minimal_thread_data.author.display_name)}
                   </div>
               </>
           }

       }
       if !self.is_selected {
           html! {
                <li class="forum-list-element",>
                    {element_internals(&self.minimal_thread_data)}
                </li>
           }
       } else {
           html! {
                <li class=("forum-list-element","list-element-selected"),>
                    {element_internals(&self.minimal_thread_data)}
                </li>
           }
       }

   }
}
