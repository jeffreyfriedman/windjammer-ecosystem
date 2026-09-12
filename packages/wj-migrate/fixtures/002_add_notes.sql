-- Smoke migration 002
ALTER TABLE smoke_items ADD COLUMN IF NOT EXISTS notes TEXT;
