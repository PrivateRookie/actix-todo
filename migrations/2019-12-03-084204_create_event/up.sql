CREATE TABLE events (
  id INTEGER NOT NULL PRIMARY KEY,
  uid VARCHAR NOT NULL,
  content TEXT NOT NULL,
  finished BOOLEAN NOT NULL,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
)