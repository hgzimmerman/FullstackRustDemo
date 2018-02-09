CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  author_id SERIAL REFERENCES users(id),
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)

