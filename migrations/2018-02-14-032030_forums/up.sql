CREATE TABLE forums (
    id SERIAL PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    forum_id INTEGER REFERENCES forums(id) NOT NULL,
    author_id INTEGER REFERENCES users(id) NOT NULL,
    created_date TIMESTAMP NOT NULL,
    locked BOOLEAN NOT NULL,
    archived BOOLEAN NOT NULL,
    title VARCHAR NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    thread_id INTEGER REFERENCES threads(id) NOT NULL,
    author_id INTEGER REFERENCES users(id) NOT NULL,
    parent_id INTEGER REFERENCES posts(id), 
    created_date TIMESTAMP NOT NULL,
    modified_date TIMESTAMP,
    content VARCHAR NOT NULL,
    censored BOOLEAN NOT NULL
)