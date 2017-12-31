use controller::Msg;
use controller::BucketMsg;
use yew::html::*;
use views::Viewable;

use models::Question;
use models::BucketModel;


//
//pub fn ask_question_view(model: &BucketModel) -> Html<Msg> {
//    html!{
//        <div>
////            <div class=("input-group", "mb-3"),>
//                <textarea
//                    class="form-control",
//                    placeholder="Ask a new question!",
//                    value=&model.new_question_input,
//                    oninput=|e: InputData| Msg::BucketQuestion(BucketMsg::BuildQuestion(e.value)),
//                    onkeypress=|e: KeyData| {
//                        if e.key == "Enter" { Msg::BucketQuestion(BucketMsg::AddQuestion) } else { Msg::NoOp }
//                    },
//                />
////                <div class=("input-group-append"),>
////                    <button class=("btn", "btn-success"), onclick=move |_| Msg::BucketQuestion(BucketMsg::AddQuestion),> { "Ask" }</button>
////                </div>
////            </div>
//        </div>
//    }
//}

pub fn ask_question_view(model: &BucketModel) -> Html<Msg> {
    html!{
        <div>
            <div class=("input-group", "mb-3"),>
                <input
                    class="form-control",
                    placeholder="Ask a question!",
                    value=&model.answer_input,
                    oninput=|e: InputData| Msg::BucketQuestion(BucketMsg::BuildQuestion(e.value)),
                    onkeypress=|e: KeyData| {
                        if e.key == "Enter" { Msg::BucketQuestion(BucketMsg::AddQuestion) } else { Msg::NoOp }
                    },
                />
                <div class=("input-group-append"),>
                    <button class=("btn", "btn-success"), onclick=move |_| Msg::BucketQuestion(BucketMsg::AddQuestion),> { "Bucket!" }</button>
                </div>
            </div>


        </div>
    }
}
