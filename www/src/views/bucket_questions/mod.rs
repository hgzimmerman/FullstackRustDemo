use controller::Msg;
use yew::html::Html;

use models::BucketModel;
use views::Viewable;

mod active_question;
mod question_input;

use self::active_question::active_question_view;
use self::question_input::ask_question_view;

impl Viewable<Msg> for BucketModel {
    fn view(&self) -> Html<Msg> {

        html!{
            <div class=("container"),>
                { active_question_view(&self) }

                <div class=("invisible"),> // This gets converted to a button for some reason
                </div> // TODO submit a bug report to Yew for this.
                // It occurs in both wasm-unknown-unknown and asmjs-unknown-emscripten

                { ask_question_view(&self) }
            </div>
        }
    }
}