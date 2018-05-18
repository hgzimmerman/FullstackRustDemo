use yew::prelude::*;
use datatypes::bucket::BucketData;
use datatypes::question::QuestionData;
use datatypes::question::NewQuestionData;
use Context;
use util::loadable::Loadable;
use util::loading::LoadingType;
use util::uploadable::Uploadable;
use util::input::InputState;
use util::input::Input;

use components::button::Button;
use datatypes::answer::AnswerData;




#[derive(Debug, Default, Clone)]
struct QuestionPackage {
    question_data: QuestionData,
    answer: InputState
}

#[derive(Debug, Default, Clone)]
struct NewQuestion {
    question_text: InputState
}

#[derive(Debug, Default)]
pub struct BucketLobby {
    bucket_data: BucketData,
    active_question: Loadable<QuestionPackage>,
    new_question: Uploadable<NewQuestion>,
    prior_questions_and_answers: Loadable<Vec<QuestionData>>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: BucketData
}

pub enum Msg {
    DrawQuestion,
    UpdateAnswer(InputState),
    SubmitAnswer,
    UpdateNewQuestion(InputState),
    SubmitNewQuestion
}

impl Component<Context> for BucketLobby {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        BucketLobby {
            bucket_data: props.bucket_data,
            ..Default::default()
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            DrawQuestion => context.log("Draw question"),
            UpdateAnswer(_) => context.log("Update Answer"),
            SubmitAnswer => context.log("Submit answer"),
            UpdateNewQuestion(_) => context.log("Update question"),
            SubmitNewQuestion => context.log("Submit question"),
        }
        true
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        // TODO, this is lazy. properly update this.
        *self = BucketLobby {
            bucket_data: props.bucket_data,
            ..Default::default()
        };
        true
    }
}
impl Renderable<Context, BucketLobby> for BucketLobby {
    fn view(&self) -> Html<Context, BucketLobby> {

        let empty_question = html! {
            <Button: title="Draw Question", onclick=|_| Msg::DrawQuestion, />
        };

        fn failed_question_view(question: &Option<String>) -> Html<Context, BucketLobby> {
            html! {
                <div>
                    <Button: title="Draw Question", onclick=|_| Msg::DrawQuestion, />
                </div>
            }
        }

        html!{
            // TODO, investigate if these no-scroll classes are needed.
            <div class=("full-height", "full-width", "no-scroll"),>
                <div class=("flexbox-horiz", "full-height", "no-scroll"),> // (Question container and answers container) container
                    <div class=("flexbox-vert", "questions-container", "scrollable", "flexbox-test"),> // Answer question and new question container

                        <div class=("full-height", "full-width", "flexbox-center"),>
                            <div class="question-card",> // Answer question card
                                {self.active_question.restricted_custom_view(
                                    empty_question,
                                    LoadingType::Fidget{diameter: 100},
                                    QuestionPackage::view,
                                    failed_question_view
                                )}
                            </div>
                        </div>

                        <div class=("full-height","full-width", "flexbox-center"),>
                            <div class="question-card",> // new question card
                                {
                                    self.new_question.default_view(NewQuestion::view)
                                }
                            </div>
                        </div>

                    </div>
                    <div class=("flexbox-vert", "answers-container", "scrollable"),>
                        {
                            self.prior_questions_and_answers.default_view(Vec::<QuestionData>::view)
                        }
                    </div>
                </div>
            </div>
        }
    }
}

impl Renderable<Context, BucketLobby> for QuestionPackage {
    fn view(&self) -> Html<Context, BucketLobby> {
        html! {
            <>
                <div>
                    <h4>
                        {&self.question_data.question_text}
                    </h4>
                </div>

                <Input:
                    placeholder="Answer",
                    input_state=&self.answer,
                    on_change=|a| Msg::UpdateAnswer(a),
                    on_enter=|_| Msg::SubmitAnswer,
                />
                <Button: title="Submit Answer", onclick=|_| Msg::SubmitAnswer, />
            </>
        }
    }
}

impl Renderable<Context, BucketLobby> for NewQuestion {
    fn view(&self) -> Html<Context, BucketLobby> {
        html! {
            <>
                <div>
                    <h4>
                        {"New Question"}
                    </h4>
                </div>

                <Input:
                    placeholder="New Question",
                    input_state=&self.question_text,
                    on_change=|a| Msg::UpdateNewQuestion(a),
                    on_enter=|_| Msg::SubmitNewQuestion,
                />
                <Button: title="Add Question To Bucket", onclick=|_| Msg::SubmitNewQuestion, />
            </>
        }
    }
}

impl Renderable<Context, BucketLobby> for AnswerData {
    fn view(&self) -> Html<Context, BucketLobby> {
        html! {
            <div>
                {self.answer_text.clone().unwrap_or("".into())} // TODO possible misuse of clone here
                {&self.author.display_name}
            </div>
        }
    }
}

impl Renderable<Context, BucketLobby> for QuestionData {
    fn view(&self) -> Html<Context, BucketLobby> {
        fn answers(answers: &Vec<AnswerData>) -> Html<Context, BucketLobby> {
             html! {
                {for answers.iter().map(AnswerData::view)}
             }
        }

        html! {
            <div>
                <div>
                    {&self.question_text}
                </div>
                <div>
                    {"Answers, if there are any"} // TODO actually show the answers
                </div>

                <Button: title="Put back in Bucket", onclick=|_| Msg::SubmitNewQuestion, />
            </div>
        }
    }
}

impl Renderable<Context, BucketLobby> for Vec<QuestionData> {
    fn view(&self) -> Html<Context, BucketLobby> {
        fn answered_questions(questions: &Vec<QuestionData>) -> Html<Context, BucketLobby> {
             html! {
                {for questions.iter().map(QuestionData::view)}
             }
        }

        html! {
            <div class=("full-height"),>
                {answered_questions(self)}
            </div>
        }
    }
}
