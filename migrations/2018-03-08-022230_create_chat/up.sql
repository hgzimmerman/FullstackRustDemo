CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    chat_name VARCHAR NOT NULL,
    leader_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE junction_chat_users (
    id SERIAL PRIMARY KEY,
    chat_id INTEGER NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    chat_id INTEGER NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    reply_id INTEGER REFERENCES messages(id), -- Not all messages are replies
    message_content VARCHAR NOT NULL,
    read_flag BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL
);
