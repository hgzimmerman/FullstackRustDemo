CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR UNIQUE NOT NULL,
  display_name VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL,
  tombstone BOOLEAN NOT NULL DEFAULT 'f'
)