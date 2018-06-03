CREATE TABLE buckets (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_name VARCHAR UNIQUE NOT NULL,
    is_public_until TIMESTAMP
);

-- A junction table between users and buckets
CREATE TABLE junction_bucket_users (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_id UUID NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    owner BOOLEAN NOT NULL,
    approved BOOLEAN NOT NULL
);


CREATE TABLE questions (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_id UUID NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question_text VARCHAR NOT NULL,
    on_floor BOOLEAN NOT NULL
);

CREATE TABLE answers (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    answer_text VARCHAR -- The "answer" entity doesn't actually need to contain text.
);