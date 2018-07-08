use yew::prelude::*;
use datatypes::bucket::BucketData;
use datatypes::question::QuestionData;
use datatypes::question::QuestionLocation;

use util::loadable::Loadable;
use util::loading::LoadingType;
use util::uploadable::Uploadable;
use util::input::InputState;
use util::input::Input;

use util::button::Button;
use datatypes::answer::AnswerData;


use wire::question::QuestionResponse;
use wire::answer::AnswerResponse;
use wire::question::NewQuestionRequest;
use wire::answer::NewAnswerRequest;

use util::input::InputValidator;
use util::link::Link;

use identifiers::question::QuestionUuid;
use identifiers::bucket::BucketUuid;

use requests::BucketRequest;
use common::fetch::Networking;
use common::fetch::FetchResponse;

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

//#[derive(Default)]
pub struct BucketLobby {
    bucket_data: BucketData,
    active_question: Loadable<Uploadable<QuestionPackage>>,
    new_question: Uploadable<NewQuestion>,
    prior_questions_and_answers: Loadable<QuestionList>,
    networking: Networking,
    link: ComponentLink<BucketLobby>
}


impl BucketLobby {
    fn get_prior_questions_and_answers(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(
            BucketRequest::GetQuestions{bucket_uuid},
            |r: FetchResponse<Vec<QuestionResponse>>| {
                Msg::HandlePriorQuestionResponse(r
                    .map(|vec|
                        vec
                            .into_iter()
                            .map(QuestionData::from)
                            .collect()
                    )
                )
            },
            &self.link
        );
    }
    fn get_random_question(&mut self, bucket_uuid: BucketUuid) {
        self.networking.fetch(
            BucketRequest::GetRandomQuestion{bucket_uuid},
            |r: FetchResponse<QuestionResponse>| {
                // Convert the question response to question data,
                // move it to a quesiton package,
                // Wrap it in an uploadable wrapper
                let r = r
                    .map(QuestionData::from)
                    .map(|question_data| {
                        QuestionPackage {
                            question_data,
                            answer: InputState::default(),
                        }
                    })
                    .map(Uploadable::NotUploaded);
                Msg::HandleDrawRandomQuestionResponse(r)
            },
            &self.link
        );

    }
    fn post_new_question(&mut self, new_question_request: NewQuestionRequest, /* new_question: &mut Uploadable<NewQuestion>,*/) {
        self.networking.fetch(
            BucketRequest::CreateQuestion(new_question_request),
            |r: FetchResponse<QuestionResponse>| Msg::HandleSubmitNewQuestionResponse(r.map(|_|())),
            &self.link
        );

    }

    fn post_answer_to_question(&mut self, new_answer_request: NewAnswerRequest /*question_package: &mut Uploadable<QuestionPackage>*/) {
       self.networking.fetch(
            BucketRequest::AnswerQuestion(new_answer_request),
            |r: FetchResponse<AnswerResponse>| Msg::HandleSubmitAnswerResponse(r.map(|_|())),
            &self.link
        );

    }

    fn put_question_back_in_bucket(&mut self, question_uuid: QuestionUuid) {
        self.networking.fetch(
            BucketRequest::PutQuestionBackInBucket{question_uuid},
            |r| Msg::HandlePutOldQuestionBackInBucketResponse(r),
            &self.link
         );


    }

    fn delete_question(&mut self, question_uuid: QuestionUuid, ) {
        self.networking.fetch(
            BucketRequest::DeleteQuestion{question_uuid},
            |r| Msg::HandleDiscardQuestionResponse(r),
            &self.link
         );
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: BucketData
}

pub enum Msg {
    DrawRandomQuestion,
    HandleDrawRandomQuestionResponse(FetchResponse<Uploadable<QuestionPackage>>),
    UpdateAnswer(InputState),
    SubmitAnswer,
    HandleSubmitAnswerResponse(FetchResponse<()>),
    UpdateNewQuestion(InputState),
    SubmitNewQuestion,
    HandleSubmitNewQuestionResponse(FetchResponse<()>),
    ResetCreateQuestionText,
    HandlePriorQuestionResponse(FetchResponse<Vec<QuestionData>>),
    PutOldQuestionBackInBucket{question_uuid: QuestionUuid},
    HandlePutOldQuestionBackInBucketResponse(FetchResponse<QuestionUuid>),
    DiscardQuestion,
    HandleDiscardQuestionResponse(FetchResponse<QuestionUuid>),
    SetListFilter(QuestionLocation),
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

impl Component for BucketLobby {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut bucket = BucketLobby {
            bucket_data: props.bucket_data,
            active_question: Loadable::default(),
            new_question: Uploadable::default(),
            prior_questions_and_answers: Loadable::default(),
            networking: Networking::new(&link),
            link
        };

        let bucket_uuid = bucket.bucket_data.uuid;
        bucket.get_prior_questions_and_answers(bucket_uuid);

        bucket
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;
        match msg {
            DrawRandomQuestion => {
                let bucket_uuid = self.bucket_data.uuid;
                self.get_random_question(bucket_uuid)
            },
            HandleDrawRandomQuestionResponse(response) => {
                self.active_question = Loadable::from_fetch_response(response);
            }
            UpdateAnswer(input) => {
                if let Loadable::Loaded(ref mut question_package) = self.active_question {
                    question_package.as_mut().answer = input;
                } else {
                    error!("Error, should not be able to update answer if question not loaded.")
                }
            }
            SubmitAnswer => {
                let request_option: Option<NewAnswerRequest> = self.active_question.as_option().map(|question_package| {
                     let answer_text = if question_package.as_ref().answer.inner_text().len() > 0 {
                        Some(question_package.as_ref().answer.inner_text())
                    } else {
                        None
                    };

                    NewAnswerRequest {
                        question_uuid: question_package.as_ref().question_data.uuid,
                        answer_text
                    }
                });

                if let Some(new_answer_request) = request_option {
                    self.post_answer_to_question(new_answer_request)
                } else {
                    error!("Error, should not be able to submit an answer for an unloaded question.")
                }
            },
            HandleSubmitAnswerResponse(response) => {
                use self::FetchResponse::*;
                match response {
                    Success(_) => {
                        let bucket_uuid = self.bucket_data.uuid;
                        self.get_prior_questions_and_answers( bucket_uuid);
                        self.active_question = Loadable::Unloaded
                    }
                    Error(_) => {
                        if let Some(ref mut active_uploadable) = self.active_question.as_mut_option() {
                            active_uploadable.set_failed("failed to submit Answer")
                        }
                    },
                    Started => {
                        if let Some(ref mut active_question_uploadable) = self.active_question.as_mut_option() {
                            active_question_uploadable.set_uploading()
                        }
                    },
                }
            }
            UpdateNewQuestion(input) => self.new_question.as_mut().question_text = input,
            SubmitNewQuestion => {
                let question_text = self.new_question.as_ref().question_text.inner_text();
                let bucket_uuid = self.bucket_data.uuid;
                let new_question_request = NewQuestionRequest {
                    bucket_uuid,
                    question_text
                };
                self.post_new_question(new_question_request);
            },
            HandleSubmitNewQuestionResponse(response) => {
                use self::FetchResponse::*;
                match response {
                    Success(_) => {
                        self.update(ResetCreateQuestionText);
                    },
                    Error(_) => self.new_question.set_failed("failed to submit question"),
                    Started => self.new_question.set_uploading(),
                };
            }
            ResetCreateQuestionText => self.new_question = Uploadable::default(),
            HandlePriorQuestionResponse(response) => {
               match response {
                    FetchResponse::Success(questions) => {
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
                    FetchResponse::Error(_) => {
                        warn!("Get prior questions failed");
                        self.prior_questions_and_answers = Loadable::Failed(Some(String::from("Could not load old questions")))
                    },
                    FetchResponse::Started => self.prior_questions_and_answers = Loadable::Loading,
                }
            }
            PutOldQuestionBackInBucket{question_uuid} => self.put_question_back_in_bucket(question_uuid),
            HandlePutOldQuestionBackInBucketResponse(response) => {
                match response {
                    FetchResponse::Success(question_uuid) => {
                        if let Loadable::Loaded(ref mut q_list) = self.prior_questions_and_answers {
                            // Set the question to say it is in the bucket now locally,
                            // instead of fetching an up to date version of the list.
                            q_list.list
                                .iter_mut()
                                .for_each(|x| {
                                    if x.uuid == question_uuid {
                                        x.location = QuestionLocation::Bucket
                                    }
                                })
                        }
                    }
                    FetchResponse::Error(_) => error!("failed to put question back in bucket"),
                    FetchResponse::Started => {}
                }
            }
//            QuestionPutBackInBucketSuccess {question_uuid} => {
//                if let Loadable::Loaded(ref mut q_list) = self.prior_questions_and_answers {
//                    // Set the question to say it is in the bucket now locally,
//                    // instead of fetching an up to date version of the list.
//                    q_list.list
//                        .iter_mut()
//                        .for_each(|x| {
//                            if x.uuid == question_uuid {
//                                x.location = QuestionLocation::Bucket
//                            }
//                        })
//                }
//            },
            DiscardQuestion => {
                if let Some(question_uuid) = self.active_question.as_option().map(|x|x.as_ref().question_data.uuid) {
                    self.delete_question(question_uuid)
                }
            }
            HandleDiscardQuestionResponse(response) => {
                match response {
                    FetchResponse::Success(question_uuid) => {
                        if let Loadable::Loaded(ref mut old_list) = self.prior_questions_and_answers {
                            old_list.list.retain(|x| x.uuid != question_uuid) // Remove the question from the local list of questions.
                        }
                    }
                    FetchResponse::Error(_) => warn!("Failed to discard question"),
                    FetchResponse::Started => {}
                }

            }
            SetListFilter(location) => {
                if let Loadable::Loaded(ref mut old_list) = self.prior_questions_and_answers {
                    old_list.filter = location
                }
            }
            NoOp => return false
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {

        self.bucket_data = props.bucket_data;
        self.active_question = Loadable::default();
        self.new_question = Uploadable::default();
        self.prior_questions_and_answers = Loadable::default();

        let bucket_uuid = self.bucket_data.uuid;
        self.get_prior_questions_and_answers( bucket_uuid);
        true
    }
}
impl Renderable<BucketLobby> for BucketLobby {
    fn view(&self) -> Html< BucketLobby> {

        let empty_question = html! {
            <div class=("full-height", "full-width", "flexbox-center"),>
                <Button: title="Draw Question", onclick=|_| Msg::DrawRandomQuestion, />
            </div>
        };

        fn failed_question_view(error_msg: &Option<String>) -> Html<BucketLobby> {
            if let Some(error_msg) = error_msg {
                html!{
                    <div class=("full-height", "full-width", "flexbox-center"),>
                        {error_msg}
                        <Button: title="Draw Question", onclick=|_| Msg::DrawRandomQuestion, />
                    </div>
                }
            } else {
                html! {
                    <div class=("full-height", "full-width", "flexbox-center"),>
                        <Button: title="Draw Question", onclick=|_| Msg::DrawRandomQuestion, />
                    </div>
                }
            }
        }

        /// This is needed in order to call a default_view within another default_view
        fn uploadable_question_shim_fn(question_package: &Uploadable<QuestionPackage>) -> Html<BucketLobby> {
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

impl Renderable<BucketLobby> for QuestionPackage {
    fn view(&self) -> Html<BucketLobby> {
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
                    <Button: title="Replace Question", onclick=|_| Msg::DrawRandomQuestion, />
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

impl Renderable<BucketLobby> for NewQuestion {
    fn view(&self) -> Html<BucketLobby> {
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

impl Renderable<BucketLobby> for AnswerData {
    fn view(&self) -> Html<BucketLobby> {
        html! {
            <div>
                {&format!("{}: ",self.author.display_name)}
                {self.answer_text.clone().unwrap_or("".into())} // TODO possible misuse of clone here
            </div>
        }
    }
}

impl Renderable<BucketLobby> for QuestionData {
    fn view(&self) -> Html<BucketLobby> {
        fn answers(answers: &Vec<AnswerData>) -> Html<BucketLobby> {
             html! {
                {for answers.iter().map(AnswerData::view)}
             }
        }

        let question_uuid: QuestionUuid = self.uuid;
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
                                    <Link<()>: name="Return to bucket", callback=move |_| Msg::PutOldQuestionBackInBucket{question_uuid}, classes="small-link", />
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

impl Renderable<BucketLobby> for QuestionList {
    fn view(&self) -> Html<BucketLobby> {

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
