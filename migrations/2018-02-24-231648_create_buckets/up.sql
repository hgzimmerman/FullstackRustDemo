CREATE TABLE buckets (
    id SERIAL PRIMARY KEY,
    bucket_name VARCHAR UNIQUE NOT NULL
);



CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    bucket_id INTEGER REFERENCES buckets(id) NOT NULL,
    author_id INTEGER REFERENCES users(id) NOT NULL,
    question_text VARCHAR NOT NULL
);

CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    question_id INTEGER REFERENCES questions(id) NOT NULL,
    author_id INTEGER REFERENCES users(id) NOT NULL,
    answer_text VARCHAR
);