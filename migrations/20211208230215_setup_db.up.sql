-- Add up migration script here
CREATE TABLE IF NOT EXISTS api_key (
  email VARCHAR(255) PRIMARY KEY,
  content TEXT
);

CREATE TABLE IF NOT EXISTS api_token (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255),
  last_used DATE NOT NULL DEFAULT CURRENT_DATE,
  content TEXT
);
