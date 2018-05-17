use yew::prelude::*;
use datatypes::bucket::BucketData;
use datatypes::question::QuestionData;
use datatypes::question::NewQuestionData;
use Context;
use util::loadable::Loadable;
use util::uploadable::Uploadable;
use util::input::InputState;


#[derive(Debug, Default, Clone)]
struct QuestionPackage {
    question_data: QuestionData,
    answer: InputState
}

#[derive(Debug, Default)]
pub struct Bucket {
    bucket_data: BucketData,
    active_question: Loadable<QuestionPackage>,
    new_question: Uploadable<NewQuestionData>,
    prior_questions_and_answers: Loadable<Vec<QuestionData>>
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Props {
    pub bucket_data: BucketData
}

pub enum Msg {

}

impl Component<Context> for Bucket {
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        unimplemented!();
        Bucket{
            bucket_data: props.bucket_data,
            ..Default::default()
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        unimplemented!();
    }

    fn change(&mut self, props: Self::Properties, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}
impl Renderable<Context, Bucket> for Bucket {
    fn view(&self) -> Html<Context, Bucket> {
        unimplemented!();
//        html! {
//
//        }
    }
}
