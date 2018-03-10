CREATE TABLE forums (
    id SERIAL PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    forum_id INTEGER NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_date TIMESTAMP NOT NULL,
    locked BOOLEAN NOT NULL,
    archived BOOLEAN NOT NULL,
    title VARCHAR NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    thread_id INTEGER NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id INTEGER REFERENCES posts(id) ON DELETE SET NULL, -- Not all posts have a parent, if the parent is deleted, set this to null
    created_date TIMESTAMP NOT NULL,
    modified_date TIMESTAMP,
    content VARCHAR NOT NULL,
    censored BOOLEAN NOT NULL
)