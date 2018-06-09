CREATE TABLE chats (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    chat_name VARCHAR NOT NULL,
    leader_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE
);

CREATE TABLE junction_chat_users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    chat_uuid UUID NOT NULL REFERENCES chats(uuid) ON DELETE CASCADE,
    user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE
);

CREATE TABLE messages (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    chat_uuid Uuid NOT NULL REFERENCES chats(uuid) ON DELETE CASCADE,
    -- If the message this message is replying to is deleted, then this message will still exist
    -- only it won't point to anything.
    reply_uuid UUID REFERENCES messages(uuid) ON DELETE SET NULL, -- Not all messages are replies
    message_content VARCHAR NOT NULL,
    read_flag BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL
);
