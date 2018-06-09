--CREATE TYPE UserRole AS ENUM ('unprivileged', 'admin', 'moderator');

CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    user_name VARCHAR UNIQUE NOT NULL,
    display_name VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    locked TIMESTAMP,
    failed_login_count Integer NOT NULL,
    banned BOOLEAN NOT NULL,
    roles Integer[] NOT NULL
  -- updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- SELECT diesel_manage_updated_at('users');