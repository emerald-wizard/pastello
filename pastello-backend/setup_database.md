## Setting Up the Postgres Database

Using Postgress.app

Open the postgres database by double clicking on it.

Run the following command.

```
-- Create a database for the entire app
CREATE DATABASE pasteltime;
CREATE USER pastel_user WITH PASSWORD 'secure_password';
GRANT CONNECT ON DATABASE pasteltime TO pastel_user;

-- Create schemas for logical separation
CREATE SCHEMA auth;       -- For user authentication tables
CREATE SCHEMA tasks;      -- For task-related tables
CREATE SCHEMA analytics;  -- For reporting/analytics tables
CREATE SCHEMA games;  -- For reporting/analytics tables

GRANT USAGE ON SCHEMA auth         TO pastel_user;       -- For user authentication tables
GRANT USAGE ON SCHEMA game        TO pastel_user;  -- For reporting/analytics tables
GRANT USAGE ON SCHEMA tasks        TO pastel_user;      -- For task-related tables
GRANT USAGE ON SCHEMA analytics    TO pastel_user;  -- For reporting/analytics tables

GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA auth         TO pastel_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA games        TO pastel_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA tasks        TO pastel_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA analytics    TO pastel_user;

ALTER DEFAULT PRIVILEGES IN SCHEMA auth         GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA games        GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA tasks        GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO pastel_user;


-- In the auth schema
CREATE TABLE auth.users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE auth.UserSessions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES auth.Users(id),
  session_token VARCHAR(255) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  last_active TIMESTAMP
);

-- Game Schema
CREATE SCHEMA game;

CREATE TABLE game.Games (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game.GameSessions (
  id SERIAL PRIMARY KEY,
  game_id INTEGER NOT NULL REFERENCES game.Games(id),
  status VARCHAR(50) NOT NULL,  -- e.g., 'waiting', 'active', 'finished'
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE game.GameSessionUsers (
  id SERIAL PRIMARY KEY,
  game_session_id INTEGER NOT NULL REFERENCES game.GameSessions(id),
  user_id INTEGER NOT NULL REFERENCES auth.Users(id),
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(game_session_id, user_id)
);

```