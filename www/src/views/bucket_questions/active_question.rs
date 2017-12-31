use controller::Msg;
use controller::BucketMsg;
use yew::html::*;
use views::Viewable;

use models::Question;
use models::BucketModel;



pub fn active_question_view(model: &BucketModel) -> Html<Msg> {
    match model.active_question {
        Some(ref q) => {
            html! {
                <div>
                    <h3>
                        <h3>
                        {format!( "Q:  {}", q.question)}
                        </h3>
                    </h3>
                   <div class=("input-group", "mb-3"),>
                    <textarea
                        class="form-control",
                        placeholder="Answer the question",
                        value=&model.answer_input,
                        oninput=|e: InputData| Msg::BucketQuestion(BucketMsg::BuildAnswer(e.value)),
                        onkeypress=|e: KeyData| {
                            if e.key == "Enter" { Msg::BucketQuestion(BucketMsg::AnswerQuestion) } else { Msg::NoOp }
                        },
                    />

                </div>
                <div class=("input-group-append"),>
                    <button class=("btn", "btn-success"), onclick=move |_| Msg::BucketQuestion(BucketMsg::AnswerQuestion),> { "Answer" }</button>
                    <button class=("btn", "btn-secondary"), onclick=move |_| Msg::BucketQuestion(BucketMsg::SkipQuestion),> { "Skip" }</button>
                </div>

            </div>
            }
        },
        None => {
            html! {
                <div>
                <button
                    class=("btn", "btn-primary"),
                    onclick=move |_| Msg::BucketQuestion(BucketMsg::DrawQuestion),>
                    { "Draw the next question from the bucket." }
                </button>
                </div>
            }
        }
    }
}
