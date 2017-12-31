use views::loadable::Loadable;
use controller::{Msg, Updatable};
use yew::html::Context;
use yew::services::format::{Nothing, Json};
use yew::services::fetch::{FetchService, Method};

use models::BucketModel;
use models::Question;


pub enum BucketMsg {
    AddQuestion(Question),
    BuildQuestion(String),
    AnswerQuestion(Question),
    BuildAnswer(String),
    DrawQuestion,
    SetActiveQuestion(Question)
}

impl Updatable<BucketMsg> for BucketModel {
    fn update(&mut self, context: &mut Context<Msg>, msg: BucketMsg) {
        match msg {
            _ => unimplemented!() // todo: implement update mechanism for bucket model.
        }
    }
}