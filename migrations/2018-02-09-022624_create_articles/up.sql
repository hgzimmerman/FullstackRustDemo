CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  author_id SERIAL REFERENCES users(id),
  title VARCHAR UNIQUE NOT NULL,
  body TEXT NOT NULL,
  publish_date TIMESTAMP
)

