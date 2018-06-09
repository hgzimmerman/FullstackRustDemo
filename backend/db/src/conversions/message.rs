use message::*;
use wire::message::*;
//use wire::chat::*;
use chrono::Utc;
//use uuid::Uuid;
use identifiers::message::MessageUuid;

impl From<MessageData> for MessageResponse {
    fn from(data: MessageData) -> MessageResponse {
        MessageResponse {
            uuid: MessageUuid(data.message.uuid),
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
            author_uuid: request.author_uuid.0,
            chat_uuid: request.chat_uuid.0,
            reply_uuid: request.reply_uuid,
            create_date: Utc::now().naive_utc(),
            message_content: request.content,
            read_flag: false, // message has not yet been read by another user
        }
    }
}
