CREATE TABLE IF NOT EXISTS sessions
  ( uuid TEXT NOT NULL PRIMARY KEY
  , user TEXT NOT NULL
  , expired BOOLEAN NOT NULL DEFAULT FALSE
  );