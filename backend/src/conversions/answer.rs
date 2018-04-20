use db::answer::*;
use requests_and_responses::answer::*;

impl From<AnswerData> for AnswerResponse {
    fn from(data: AnswerData) -> AnswerResponse {
        AnswerResponse {
            id: data.answer.id,
            answer_text: data.answer.answer_text,
            author: data.user.into(),
        }
    }
}

impl From<NewAnswerRequest> for NewAnswer {
    fn from(request: NewAnswerRequest) -> NewAnswer {
        NewAnswer {
            answer_text: request.answer_text,
            author_id: request.author_id,
            question_id: request.question_id,
        }
    }
}
