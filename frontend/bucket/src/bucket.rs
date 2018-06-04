use yew::prelude::*;
use datatypes::bucket::BucketData;
use datatypes::question::QuestionData;
//use datatypes::question::NewQuestionData;
use datatypes::question::QuestionLocation;

use Context;
use util::loadable::Loadable;
use util::loading::LoadingType;
use util::uploadable::Uploadable;
use util::input::InputState;
use util::input::Input;

use util::button::Button;
use datatypes::answer::AnswerData;

use yew::format::Json;
use yew::services::fetch::Response;
use yew::services::fetch::FetchTask;
use failure::Error;
use context::networking::RequestWrapper;


use wire::question::QuestionResponse;
use wire::answer::AnswerResponse;
use wire::question::NewQuestionRequest;
use wire::answer::NewAnswerRequest;

use util::input::InputValidator;
use util::link::Link;

use identifiers::question::QuestionUuid;
use identifiers::bucket::BucketUuid;

#[derive(Debug, Default, Clone)]
pub struct QuestionPackage {
    question_data: QuestionData,
    answer: InputState
}

#[derive(Debug, Default, Clone)]
pub struct QuestionList {
    list: Vec<QuestionData>,
    filter: QuestionLocation //show either questions in the bucket or on the floor in the righthand pane.
}

#[derive(Debug, Default, Clone)]
struct NewQuestion {
    question_text: InputState
}
impl NewQuestion {
    fn validator(text: String) -> Result<String, String> {
        if text.len() < 1 {
            return Err("New question must contain some text".into())
        }
        Ok(text)
    }
}

#[derive(Default)]
pub struct BucketLobby {
    bucket_data: BucketData,
    active_question: Loadable<Uploadable<QuestionPackage>>,
    new_question: Uploadable<NewQuestion>,
    prior_questions_and_answers: Loadable<QuestionList>,
    misc_ft: Option<FetchTask> // Fetch task for which no loading animation is assigned. only one is expected to run at a time, or invalidation of a prior ft is ok.
}


impl BucketLobby {
    fn get_prior_questions_and_answers(prior_questions: &mut Loadable<QuestionList>, bucket_id: BucketUuid, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<Vec<QuestionResponse>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::PriorQuestionsReady(
                        data.unwrap()
                            .into_iter()
                            .map(QuestionData::from)
                            .collect()
                    )
                } else {
                    Msg::PriorQuestionsFailed
                }
            },
        );

        context.make_request_and_set_ft(
            prior_questions,
            RequestWrapper::GetQuestions{bucket_id},
            callback,
        );
    }
    fn get_random_question(question_package: &mut Loadable<Uploadable<QuestionPackage>>, bucket_id: BucketUuid, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<QuestionResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let question_data = data.map(QuestionData::from).unwrap();
                    let question_package = QuestionPackage {
                        question_data,
                        answer: InputState::default(),
                    };
                    Msg::GetRandomQuestionReady(
                        question_package
                    )
                } else {
                    Msg::GetRandomQuestionFailed
                }
            },
        );

        context.make_request_and_set_ft(
            question_package,
            RequestWrapper::GetRandomQuestion{bucket_id},
            callback,
        );
    }
    fn post_new_question(new_question: &mut Uploadable<NewQuestion>, bucket_uuid: BucketUuid, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<QuestionResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let _question_data = data.map(QuestionData::from).unwrap();
                    Msg::ResetCreateQuestionText
                } else {
                    Msg::CreateQuestionFailed
                }
            },
        );

        let question_text = new_question.as_ref().question_text.inner_text();
        let new_question_request = NewQuestionRequest {
            bucket_uuid,
            question_text
        };

        context.make_request_and_set_ft(
            new_question,
            RequestWrapper::CreateQuestion( new_question_request),
            callback,
        );
    }

    fn post_answer_to_question(question_package: &mut Uploadable<QuestionPackage>, context: &mut Env<Context, Self>) {
        let callback = context.send_back(
            |response: Response<Json<Result<AnswerResponse, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
//                    let question_data = data.map(QuestionData::from).unwrap();
                    Msg::SendAnswerSuccess
                } else {
                    Msg::SendAnswerFail
                }
            },
        );


        let answer_text = if question_package.as_ref().answer.inner_text().len() > 0 {
            Some(question_package.as_ref().answer.inner_text())
        } else {
            None
        };

        let request = NewAnswerRequest {
            question_uuid: question_package.as_ref().question_data.id,
            answer_text
        };

        context.make_request_and_set_ft(
            question_package,
            RequestWrapper::AnswerQuestion(request),
            callback,
        );
    }

    fn put_question_back_in_bucket(question_id: QuestionUuid, context: &mut Env<Context, Self>) -> Option<FetchTask> {
        let callback = context.send_back(
            |response: Response<Json<Result<QuestionUuid, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let question_id: QuestionUuid = data.unwrap();
                    Msg::QuestionPutBackInBucketSuccess {question_id}
                } else {
                    Msg::QuestionPutBackInBucketFailed
                }
            },
        );

        let ft = context.make_request(
            RequestWrapper::PutQuestionBackInBucket{question_id},
            callback,
        ).expect("user logged in"); // TODO refactor this.
        Some(ft)
    }

    fn delete_question(question_id: QuestionUuid, context: &mut Env<Context, Self>) -> Option<FetchTask> {
        let callback = context.send_back(
            |response: Response<Json<Result<QuestionUuid, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    let question_id: QuestionUuid = data.unwrap();
                    Msg::DiscardQuestionSucceeded {question_id}
                } else {
                    Msg::DiscardQuestionFailed
                }
            },
        );

        let ft = context.make_request(
            RequestWrapper::DeleteQuestion{question_id},
            callback,
        ).expect("user logged in"); // TODO refactor this.
        Some(ft)
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: BucketData
}

pub enum Msg {
    DrawQuestion,
    GetRandomQuestionReady(QuestionPackage),
    GetRandomQuestionFailed,
    UpdateAnswer(InputState),
    SubmitAnswer,
    SendAnswerSuccess,
    SendAnswerFail,
    UpdateNewQuestion(InputState),
    SubmitNewQuestion,
    ResetCreateQuestionText,
    CreateQuestionFailed,
    PriorQuestionsReady(Vec<QuestionData>),
    PriorQuestionsFailed,
    PutOldQuestionBackInBucket{question_id: QuestionUuid},
    QuestionPutBackInBucketSuccess{question_id: QuestionUuid},
    QuestionPutBackInBucketFailed,
    DiscardQuestion,
    DiscardQuestionSucceeded {question_id: QuestionUuid},
    DiscardQuestionFailed,
    SetListFilter(QuestionLocation)
}

impl Component<Context> for BucketLobby {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let mut bucket = BucketLobby {
            bucket_data: props.bucket_data,
            ..Default::default()
        };

        Self::get_prior_questions_and_answers(&mut bucket.prior_questions_and_answers, bucket.bucket_data.id, context);

        bucket
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use self::Msg::*;
        match msg {
            DrawQuestion => Self::get_random_question(&mut self.active_question, self.bucket_data.id, context),
            GetRandomQuestionReady(question_package) => self.active_question = Loadable::Loaded(Uploadable::NotUploaded(question_package)),
            GetRandomQuestionFailed => self.active_question = Loadable::Failed(Some(String::from("Could not load question."))),
            UpdateAnswer(input) => {
                if let Loadable::Loaded(ref mut question_package) = self.active_question {
                    question_package.as_mut().answer = input;
                } else {
                    context.log("Error, should not be able to update answer if question not loaded.")
                }
            }
            SendAnswerSuccess => {
                Self::get_prior_questions_and_answers(&mut self.prior_questions_and_answers, self.bucket_data.id, context);
                self.active_question = Loadable::Unloaded
            },
            SendAnswerFail => self.active_question = Loadable::Failed(Some(String::from("Failed to submit question"))),
            SubmitAnswer => {
                if let Loadable::Loaded(ref mut question_package) = self.active_question {

                    Self::post_answer_to_question(question_package, context )
                } else {
                    context.log("Error, should not be able to submit an answer for an unloaded question.")
                }
            },
            UpdateNewQuestion(input) => self.new_question.as_mut().question_text = input,
            SubmitNewQuestion => Self::post_new_question(&mut self.new_question, self.bucket_data.id, context),
            ResetCreateQuestionText => self.new_question = Uploadable::default(),
            CreateQuestionFailed => self.new_question.set_failed("Could not create new question"),
            PriorQuestionsReady(questions) =>{
                if let Loadable::Loaded(ref mut old_list) = self.prior_questions_and_answers {
                    old_list.list = questions;
                } else {
                    let new_list = QuestionList {
                        list: questions,
                        filter: QuestionLocation::Floor
                    };
                    self.prior_questions_and_answers = Loadable::Loaded(new_list)
                }
            }
            PriorQuestionsFailed => {
                context.log("Get prior questions failed");
                self.prior_questions_and_answers = Loadable::Failed(Some(String::from("Could not load old questions")))
            }
            PutOldQuestionBackInBucket{question_id} => self.misc_ft = Self::put_question_back_in_bucket(question_id, context),
            QuestionPutBackInBucketSuccess {question_id} => {
                if let Loadable::Loaded(ref mut q_list) = self.prior_questions_and_answers {
                    // Set the question to say it is in the bucket now locally,
                    // instead of fetching an up to date version of the list.
                    q_list.list
                        .iter_mut()
                        .for_each(|x| {
                            if x.id == question_id {
                                x.location = QuestionLocation::Bucket
                            }
                        })
                }
            },
            QuestionPutBackInBucketFailed => context.log("failed to put question back in bucket"),
            DiscardQuestion => {
                if let Loadable::Loaded(ref active_question) = self.active_question {
                    self.misc_ft = Self::delete_question(active_question.as_ref().question_data.id, context)
                }
            }
            DiscardQuestionSucceeded { question_id} => {
                self.active_question = Loadable::Unloaded;
                if let Loadable::Loaded(ref mut old_list) = self.prior_questions_and_answers {
                    old_list.list.retain(|x| x.id != question_id) // Remove the question from the local list of questions.
                }
            },
            DiscardQuestionFailed => context.log("Failed to discard question"),
            SetListFilter(location) => {
                if let Loadable::Loaded(ref mut old_list) = self.prior_questions_and_answers {
                    old_list.filter = location
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {

        *self = BucketLobby {
            bucket_data: props.bucket_data,
            ..Default::default()
        };

        Self::get_prior_questions_and_answers(&mut self.prior_questions_and_answers, self.bucket_data.id, context);
        true
    }
}
impl Renderable<Context, BucketLobby> for BucketLobby {
    fn view(&self) -> Html<Context, BucketLobby> {

        let empty_question = html! {
            <div class=("full-height", "full-width", "flexbox-center"),>
                <Button: title="Draw Question", onclick=|_| Msg::DrawQuestion, />
            </div>
        };

        fn failed_question_view(error_msg: &Option<String>) -> Html<Context, BucketLobby> {
            if let Some(error_msg) = error_msg {
                html!{
                    <div class=("full-height", "full-width", "flexbox-center"),>
                        {error_msg}
                        <Button: title="Draw Question", onclick=|_| Msg::DrawQuestion, />
                    </div>
                }
            } else {
                html! {
                    <div class=("full-height", "full-width", "flexbox-center"),>
                        <Button: title="Draw Question", onclick=|_| Msg::DrawQuestion, />
                    </div>
                }
            }
        }

        /// This is needed in order to call a default_view within another default_view
        fn uploadable_question_shim_fn(question_package: &Uploadable<QuestionPackage>) -> Html<Context, BucketLobby> {
            question_package.default_view(QuestionPackage::view)
        }

        html!{
            <div class=("full-height", "full-width", "no-scroll"),>
                <div class=("flexbox-horiz", "full-height", "no-scroll"),> // (Question container and answers container) container
                    <div class=("flexbox-vert", "questions-container", "scrollable", "flexbox-test"),> // Answer question and new question container

                        <div class=("full-height", "full-width", "flexbox-center"),>
                            <div class=("question-card", "active-question-card"),> // Answer question card
                                {self.active_question.restricted_custom_view(
                                    empty_question,
                                    LoadingType::Fidget{diameter: 100},
                                    uploadable_question_shim_fn,
                                    failed_question_view
                                )}
                            </div>
                        </div>

                        <div class=("full-height","full-width", "flexbox-center"),>
                            <div class=("question-card", "new-question-card"),> // new question card
                                {
                                    self.new_question.default_view(NewQuestion::view)
                                }
                            </div>
                        </div>

                    </div>
                    <div class=("flexbox-vert", "answers-container", "scrollable"),>
                        {
                            self.prior_questions_and_answers.default_view( QuestionList::view)
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
            <div class=("full-height", "full-width","flexbox-vert"),>
                <div class=("padding-left", "padding-right", "flexbox-expand"),>
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
                </div>
                <div class=("flexbox-horiz-reverse"),>
                    <Button: title="Submit Answer", onclick=|_| Msg::SubmitAnswer, />
                    <Button: title="Replace Question", onclick=|_| Msg::DrawQuestion, />
                    {
                        // You can't delete a question which already has an answer
                        if self.question_data.answers.len() < 1 {
                            html! {
                                <Button: title="Discard", onclick=|_| Msg::DiscardQuestion, />
                            }
                        } else {
                            ::util::wrappers::empty_vdom_node()
                        }
                    }
                </div>
            </div>
        }
    }
}

impl Renderable<Context, BucketLobby> for NewQuestion {
    fn view(&self) -> Html<Context, BucketLobby> {
        html! {
            <div class=("full-height", "full-width","flexbox-vert"),>
                <div class=("padding-left", "padding-right", "flexbox-expand"),>
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
                        validator=Box::new(NewQuestion::validator as InputValidator),
                    />
                </div>
                <div class=("flexbox-horiz-reverse"),>
                    <Button: title="Add Question To Bucket", onclick=|_| Msg::SubmitNewQuestion, />
                </div>
            </>
        }
    }
}

impl Renderable<Context, BucketLobby> for AnswerData {
    fn view(&self) -> Html<Context, BucketLobby> {
        html! {
            <div>
                {&format!("{}: ",self.author.display_name)}
                {self.answer_text.clone().unwrap_or("".into())} // TODO possible misuse of clone here
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

        let question_id: QuestionUuid = self.id;
        html! {
            <div class=("flexbox-vert", "bordered", "margin-default"),>
                <div class=("flexbox-vert", "border-bottom", "padding-default"),>
                    <div class="bolded",>
                        {&self.question_text}
                    </div>
                    <div>
                        {
                            if self.location == QuestionLocation::Floor {
                                html! {
                                    <Link<()>: name="Return to bucket", callback=move |_| Msg::PutOldQuestionBackInBucket{question_id}, classes="small-link", />
                                }
                            } else {
                                ::util::wrappers::empty_vdom_node()
                            }
                        }
                    </div>
                </div>

                <div class=("padding-default"),>
                    {answers(&self.answers)}
                </div>

            </div>
        }
    }
}

impl Renderable<Context, BucketLobby> for QuestionList {
    fn view(&self) -> Html<Context, BucketLobby> {

        let floor_filter_disabled: bool = self.filter == QuestionLocation::Floor; // If Floor is already selected, disable the button
        let bucket_filter_disabled: bool = !floor_filter_disabled;

        html! {
            <div class=("full-height", "question-list"),>
                <div class=("flexbox-horiz"),>
                    <Button: title="Floor",  disabled=floor_filter_disabled, onclick=move |_| Msg::SetListFilter(QuestionLocation::Floor), />
                    <Button: title="Bucket", disabled=bucket_filter_disabled, onclick=move |_| Msg::SetListFilter(QuestionLocation::Bucket), />
                </div>
                {for self.list.iter().filter(|x| x.location == self.filter).map(QuestionData::view)}
            </div>
        }
    }
}
