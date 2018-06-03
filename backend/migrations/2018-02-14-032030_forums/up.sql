CREATE TABLE forums (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    title VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE threads (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_date TIMESTAMP NOT NULL,
    locked BOOLEAN NOT NULL,
    archived BOOLEAN NOT NULL,
    title VARCHAR NOT NULL
);

CREATE TABLE posts (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    thread_id UUID NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- Not all posts have a parent, if the parent is deleted, delete this as well,
    -- as a null parent signifies the post is the OP for a thread.
    parent_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    created_date TIMESTAMP NOT NULL,
    modified_date TIMESTAMP,
    content VARCHAR NOT NULL,
    censored BOOLEAN NOT NULL
)