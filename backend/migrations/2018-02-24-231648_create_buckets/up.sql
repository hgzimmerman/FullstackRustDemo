CREATE TABLE buckets (
    id SERIAL PRIMARY KEY,
    bucket_name VARCHAR UNIQUE NOT NULL,
    is_public BOOLEAN NOT NULL
);

-- A junction table between users and buckets
CREATE TABLE junction_bucket_users (
    id SERIAL PRIMARY KEY,
    bucket_id INTEGER NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    owner BOOLEAN NOT NULL,
    approved BOOLEAN NOT NULL
);


CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    bucket_id INTEGER NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question_text VARCHAR NOT NULL,
    on_floor BOOLEAN NOT NULL
);

CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    question_id INTEGER NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    answer_text VARCHAR -- The "answer" entity doesn't actually need to contain text.
);