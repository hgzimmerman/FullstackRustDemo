use controller::Msg;
use yew::html::Html;

use views::loadable::Loadable;
use models::{Page, NewsModel, Article, BucketModel};

pub fn view() -> Html<Msg> {
    html! {
        <nav class=("navbar","navbar-expand-lg", "navbar-light", "bg-light"),>
            <a class="navbar-brand", href="#",>{"Weekend@Joes"}</a>

             <button class="navbar-toggler", type="button", data_toggle="collapse", data_target="#navbarSupportedContent",>
                <span class="navbar-toggler-icon",></span>
             </button>

            <div class=("navbar-collapse", "collapse"), id="navbarSupportedContent",>
                <ul class="navbar-nav",>

                    <li class="nav-item",>
                        <a class="nav-link",
                            href="#",
                            onclick = move |_| {

                                Msg::SetTopLevelPage(Page::News(NewsModel{
                                    link_id: "article1".to_string() , // Needed???
                                    article: Loadable::Loaded(Article::temp())
                                }))
                            },
                         >
                            {r##""News""##}
                         </a>
                    </li>
                    <li class="nav-item",>
                        <a  class="nav-link",
                            href="#/bucket_questions",
                            onclick = move |_| Msg::SetTopLevelPage(Page::BucketQuestions(BucketModel::temp())),
                         >
                            {"Bucket Questions"}
                        </a>
                    </li>
                </ul>
            </div>
            // Right Side
            <div>
                <ul class=("navbar-nav"),>
                    <li class=("nav-item", "dropdown"),>
                        <a class=("nav-link", "dropdown-toggle"),
                            id="navbarDropdown",
                            href="#",
                            data_toggle="dropdown", // TODO this should be 'data-toggle', YEW can't parse that for now.
                            >
                            { "JOE" }
                        </a>
                        <div class=("dropdown-menu"),>
                            <a class=("dropdown-item"),> {"Settings"}</a>
                        </div>


                    </li>
                </ul>
            </div>

        </nav>
    }
}