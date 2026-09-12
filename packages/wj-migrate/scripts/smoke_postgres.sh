#!/usr/bin/env bash
# Smoke-test wj-migrate SQL against the local platform-db Postgres container.
# Usage: ./scripts/smoke_postgres.sh
# Requires: docker, healthy container named platform-db
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIX="$ROOT/fixtures"
CONTAINER="${WJ_MIGRATE_PG_CONTAINER:-platform-db}"
DB="${WJ_MIGRATE_PG_DB:-wj_migrate_smoke}"

psql_db() {
  docker exec -i "$CONTAINER" sh -c "psql -v ON_ERROR_STOP=1 -U \"\$POSTGRES_USER\" -d \"$DB\""
}

echo "==> ensuring database $DB"
docker exec "$CONTAINER" sh -c "psql -U \"\$POSTGRES_USER\" -d \"\$POSTGRES_DB\" -tc \"SELECT 1 FROM pg_database WHERE datname='$DB'\"" \
  | grep -q 1 \
  || docker exec "$CONTAINER" sh -c "psql -U \"\$POSTGRES_USER\" -d \"\$POSTGRES_DB\" -c \"CREATE DATABASE $DB;\""

echo "==> reset smoke schema"
psql_db <<'SQL'
DROP TABLE IF EXISTS smoke_items;
DROP TABLE IF EXISTS schema_migrations;
SQL

echo "==> create schema_migrations (from wj-migrate::schema_table_sql)"
psql_db <<'SQL'
CREATE TABLE IF NOT EXISTS schema_migrations (
  version INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  applied_at TEXT NOT NULL
);
SQL

echo "==> apply 001_create_smoke_items.sql"
psql_db < "$FIX/001_create_smoke_items.sql"
psql_db <<'SQL'
INSERT INTO schema_migrations (version, name, applied_at)
VALUES (1, 'create_smoke_items', '2026-08-23T00:00:00Z');
SQL

echo "==> apply 002_add_notes.sql"
psql_db < "$FIX/002_add_notes.sql"
psql_db <<'SQL'
INSERT INTO schema_migrations (version, name, applied_at)
VALUES (2, 'add_notes', '2026-08-23T00:00:01Z');
SQL

echo "==> verify"
psql_db <<'SQL'
INSERT INTO smoke_items (id, label, notes) VALUES (1, 'hello', 'from migrate smoke');
SELECT version, name FROM schema_migrations ORDER BY version;
SELECT id, label, notes FROM smoke_items;
SQL

echo "OK: wj-migrate postgres smoke passed on $CONTAINER/$DB"
