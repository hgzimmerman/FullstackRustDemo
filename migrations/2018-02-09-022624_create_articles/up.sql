CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  author_id SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR UNIQUE NOT NULL,
  slug VARCHAR UNIQUE NOT NULL,
  body TEXT NOT NULL,
  publish_date TIMESTAMP
)

