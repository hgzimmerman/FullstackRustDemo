use db::message::*;
use wire::message::*;
use chrono::Utc;

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

impl From<NewMessageRequest> for NewMessage {
    fn from(request: NewMessageRequest) -> NewMessage {
        NewMessage {
            author_id: request.author_id,
            chat_id: request.chat_id,
            reply_id: request.reply_id,
            create_date: Utc::now().naive_utc(),
            message_content: request.content,
            read_flag: false, // message has not yet been read by another user
        }
    }
}
