use db::question::*;
use wire::question::*;
use wire::answer::AnswerResponse;
use identifiers::question::QuestionUuid;
use identifiers::bucket::BucketUuid;


impl From<QuestionData> for QuestionResponse {
    fn from(data: QuestionData) -> QuestionResponse {

        QuestionResponse {
            id: QuestionUuid(data.question.id),
            bucket_id: BucketUuid(data.question.bucket_id),
            question_text: data.question.question_text,
            author: data.user.clone().into(),
            answers: data.answers
                .into_iter()
                .map(AnswerResponse::from)
                .collect(),
            on_floor: data.question.on_floor,
        }
    }
}

impl NewQuestion {
    pub fn attach_user_id(request: NewQuestionRequest, user_id: i32) -> NewQuestion {
        NewQuestion {
            bucket_id: request.bucket_id.0,
            author_id: user_id,
            question_text: request.question_text,
            on_floor: false, // by default, the question is in the bucket and not in the floor.
        }
    }
}
//impl From<NewQuestionRequest> for NewQuestion {
//    fn from(request: NewQuestionRequest) -> NewQuestion {
//        NewQuestion {
//            bucket_id: request.bucket_id,
//            author_id: request.author_id,
//            question_text: request.question_text,
//        }
//    }
//}
