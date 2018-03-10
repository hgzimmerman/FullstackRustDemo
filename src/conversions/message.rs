use db::message::*;
use requests_and_responses::message::*;

impl From<MessageData> for MessageResponse {
    fn from(data: MessageData) -> MessageResponse {
        MessageResponse {
            id: data.message.id,
            author: data.author.into(),
            reply: data.reply
                .map(|x| MessageResponse::from(*x))
                .map(Box::new),
            content: data.message.message_content,
            date: data.message.create_date,
        }
    }
}
