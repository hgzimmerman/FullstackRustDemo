use crate::chat::*;
use identifiers::chat::ChatUuid;
use wire::{
    chat::*,
    user::UserResponse,
};

impl From<ChatUserAssociationRequest> for ChatUserAssociation {
    fn from(request: ChatUserAssociationRequest) -> ChatUserAssociation {
        ChatUserAssociation {
            user_uuid: request.user_uuid.0,
            chat_uuid: request.chat_uuid.0,
        }
    }
}

impl From<ChatData> for ChatResponse {
    fn from(data: ChatData) -> ChatResponse {
        ChatResponse {
            uuid: ChatUuid(data.chat.uuid),
            name: data.chat.chat_name,
            leader: data.leader.into(),
            members: data.members.into_iter().map(UserResponse::from).collect(),
        }
    }
}

impl From<Chat> for MinimalChatResponse {
    fn from(chat: Chat) -> MinimalChatResponse {
        MinimalChatResponse {
            uuid: ChatUuid(chat.uuid),
            name: chat.chat_name,
        }
    }
}

impl From<NewChatRequest> for NewChat {
    fn from(request: NewChatRequest) -> NewChat {
        NewChat {
            chat_name: request.name,
            leader_uuid: request.leader_uuid.0,
        }
    }
}
