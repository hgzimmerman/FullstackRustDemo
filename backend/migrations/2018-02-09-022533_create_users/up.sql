--CREATE TYPE UserRole AS ENUM ('unprivileged', 'admin', 'moderator');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
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