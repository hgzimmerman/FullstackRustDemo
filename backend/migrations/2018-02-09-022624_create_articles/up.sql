CREATE TABLE articles (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    author_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    title VARCHAR UNIQUE NOT NULL,
    slug VARCHAR UNIQUE NOT NULL,
    body TEXT NOT NULL,
    publish_date TIMESTAMP
)

