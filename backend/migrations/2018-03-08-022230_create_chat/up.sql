CREATE TABLE chats (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    chat_name VARCHAR NOT NULL,
    leader_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE junction_chat_users (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    chat_id UUID NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE messages (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    chat_id Uuid NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    -- If the message this message is replying to is deleted, then this message will still exist
    -- only it won't point to anything.
    reply_id UUID REFERENCES messages(id) ON DELETE SET NULL, -- Not all messages are replies
    message_content VARCHAR NOT NULL,
    read_flag BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL
);
