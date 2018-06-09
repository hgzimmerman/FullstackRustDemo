CREATE TABLE forums (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    title VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE threads (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    forum_uuid UUID NOT NULL REFERENCES forums(uuid) ON DELETE CASCADE,
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    created_date TIMESTAMP NOT NULL,
    locked BOOLEAN NOT NULL,
    archived BOOLEAN NOT NULL,
    title VARCHAR NOT NULL
);

CREATE TABLE posts (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    thread_uuid UUID NOT NULL REFERENCES threads(uuid) ON DELETE CASCADE,
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    -- Not all posts have a parent, if the parent is deleted, delete this as well,
    -- as a null parent signifies the post is the OP for a thread.
    parent_uuid UUID REFERENCES posts(uuid) ON DELETE CASCADE,
    created_date TIMESTAMP NOT NULL,
    modified_date TIMESTAMP,
    content VARCHAR NOT NULL,
    censored BOOLEAN NOT NULL
)