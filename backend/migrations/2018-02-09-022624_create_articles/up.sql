CREATE TABLE articles (
    id UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR UNIQUE NOT NULL,
    slug VARCHAR UNIQUE NOT NULL,
    body TEXT NOT NULL,
    publish_date TIMESTAMP
)

