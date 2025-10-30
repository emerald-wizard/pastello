CREATE SCHEMA auth;  -- For reporting/analytics tables
CREATE SCHEMA game;  -- For reporting/analytics tables

GRANT USAGE ON SCHEMA auth TO pastel_user;  -- For user authentication tables
GRANT USAGE ON SCHEMA game TO pastel_user;  -- For reporting/analytics table


GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA auth         TO pastel_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA game        TO pastel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA auth  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA game GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;

-- In the auth schema
CREATE TABLE auth.users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE auth.user_sessions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES auth.Users(id),
  session_token VARCHAR(255) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  last_active TIMESTAMP
);

-- Game Schema

CREATE TABLE game.games (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game.game_sessions (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES game.Games(id),
  status VARCHAR(50) NOT NULL,  -- e.g., 'waiting', 'active', 'finished'
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game.game_session_users (
  id SERIAL PRIMARY KEY,
  game_session_id INTEGER NOT NULL REFERENCES game.game_sessions(id),
  user_id INTEGER NOT NULL REFERENCES auth.Users(id),
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(game_session_id, user_id)
);
