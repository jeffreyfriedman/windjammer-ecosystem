-- Smoke migration 001
CREATE TABLE IF NOT EXISTS smoke_items (
  id INTEGER PRIMARY KEY,
  label TEXT NOT NULL
);
