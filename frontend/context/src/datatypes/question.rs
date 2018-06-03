use datatypes::answer::AnswerData;
use datatypes::user::UserData;
use wire::question::QuestionResponse;
use identifiers::bucket::BucketUuid;
use identifiers::question::QuestionUuid;


#[derive(Clone, Debug, PartialEq)]
pub enum QuestionLocation {
    Floor,
    Bucket
}

impl Default for QuestionLocation {
    fn default() -> Self {
        QuestionLocation::Floor
    }
}

#[derive(Clone, Debug, Default)]
pub struct QuestionData {
    pub id: QuestionUuid,
    pub bucket_id: BucketUuid,
    pub question_text: String,
    pub author: UserData,
    pub answers: Vec<AnswerData>,
    pub location: QuestionLocation
}

impl From<QuestionResponse> for QuestionData {
    fn from(response: QuestionResponse) -> QuestionData {
        QuestionData {
            id: response.id,
            bucket_id: response.bucket_id,
            question_text: response.question_text,
            author: UserData::from(response.author),
            answers: response.answers.into_iter().map(AnswerData::from).collect(),
            location: if response.on_floor {QuestionLocation::Floor} else {QuestionLocation::Bucket}
        }
    }
}


#[derive(Clone, Debug, Default)]
pub struct NewQuestionData {
    pub question_text: String
}
