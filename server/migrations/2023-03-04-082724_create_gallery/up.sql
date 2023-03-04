-- Your SQL goes here
CREATE TABLE gallery_images (
  file_path TEXT NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  size INTEGER NOT NULL,
  width INTEGER,
  height INTEGER,
  format TEXT,
  updated_at TEXT NOT NULL
)
