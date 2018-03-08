CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    chat_name VARCHAR NOT NULL,
    leader_id INTEGER REFERENCES users(id) NOT NULL
);

CREATE TABLE junction_chat_users (
    id SERIAL PRIMARY KEY,
    chat_id INTEGER REFERENCES chats(id) NOT NULL,
    user_id INTEGER REFERENCES users(id) NOT NULL
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    author_id INTEGER REFERENCES users(id) NOT NULL,
    chat_id INTEGER REFERENCES chats(id) NOT NULL,
    reply_id INTEGER REFERENCES messages(id),
    message_content VARCHAR NOT NULL,
    read_flag BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL
);
