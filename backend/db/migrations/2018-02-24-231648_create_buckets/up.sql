CREATE TABLE buckets (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_name VARCHAR UNIQUE NOT NULL,
    is_public_until TIMESTAMP
);

-- A junction table between users and buckets
CREATE TABLE junction_bucket_users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_uuid UUID NOT NULL REFERENCES buckets(uuid) ON DELETE CASCADE,
    user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    owner BOOLEAN NOT NULL
);


CREATE TABLE questions (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    bucket_uuid UUID NOT NULL REFERENCES buckets(uuid) ON DELETE CASCADE,
    author_uuid UUID REFERENCES users(uuid) ON DELETE CASCADE,
    question_text VARCHAR NOT NULL,
    on_floor BOOLEAN NOT NULL
);

-- A junction table between users and their favorite questions.
CREATE TABLE junction_favorite_questions_users (
  uuid UUID PRIMARY Key NOT NULL DEFAULT gen_random_uuid(),
  question_uuid UUID NOT NULL REFERENCES questions(uuid) ON DELETE CASCADE,
  user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE
);

CREATE TABLE answers (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    question_uuid UUID NOT NULL REFERENCES questions(uuid) ON DELETE CASCADE,
    author_uuid UUID REFERENCES users(uuid) ON DELETE CASCADE,
    answer_text VARCHAR -- The "answer" entity doesn't actually need to contain text.
);