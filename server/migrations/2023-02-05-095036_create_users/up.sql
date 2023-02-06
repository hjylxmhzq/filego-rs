-- Your SQL goes here
CREATE TABLE users (
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  email TEXT NOT NULL,
  PRIMARY KEY (username)
)