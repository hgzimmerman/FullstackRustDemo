use db::chat::*;
use requests_and_responses::chat::*;
use requests_and_responses::user::UserResponse;


impl From<ChatUserAssociationRequest> for ChatUserAssociation {
    fn from(request: ChatUserAssociationRequest) -> ChatUserAssociation {
        ChatUserAssociation {
            user_id: request.user_id,
            chat_id: request.chat_id,
        }
    }
}

impl From<ChatData> for ChatResponse {
    fn from(data: ChatData) -> ChatResponse {
        ChatResponse {
            id: data.chat.id,
            name: data.chat.chat_name,
            leader: data.leader.into(),
            members: data.members
                .into_iter()
                .map(UserResponse::from)
                .collect(),
        }
    }
}

impl From<Chat> for MinimalChatResponse {
    fn from(chat: Chat) -> MinimalChatResponse {
        MinimalChatResponse {
            id: chat.id,
            name: chat.chat_name,
        }
    }
}

impl From<NewChatRequest> for NewChat {
    fn from(request: NewChatRequest) -> NewChat {
        NewChat {
            chat_name: request.name,
            leader_id: request.leader_id,
        }
    }
}
