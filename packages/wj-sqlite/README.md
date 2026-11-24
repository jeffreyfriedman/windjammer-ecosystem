# wj-sqlite

Read a SQLite database file with no SQLite library: the header, `sqlite_master`, table b-trees (interior and leaf pages), overflow chains and the record format, all in Windjammer. Use it to inspect or export a database you were handed (a CI artefact, a backup, a test fixture) as plain data. Queries and writes stay with `std::db`, which is the SQLite binding.

```windjammer
use wj_sqlite::database::read_file

let db = read_file("app.db")?
for table in db.tables()? {
    println("${table.name}: ${table.columns.join(", ")}")
}
for row in db.rows("users")? {
    println("${row.rowid} ${show(row.values[1])}")
}
```

## API

- `database`: `open(bytes)`, `read_file(path)`, then `page_count`, `page_size`, `tables`, `table(name)`, `rows(name)` (rowid order; an `INTEGER PRIMARY KEY` column carries the rowid), `scan(root_page)`, `columns_of`, `rowid_alias`
- `format`: `Value` (`Null`, `Int`, `Real`, `Text`, `Blob`), `decode_record`, `varint`, `real_be`, `show`

Checked against a database written by Python's `sqlite3` (page size 512, interior pages, a 1200-byte blob in overflow pages, quoted and constrained column definitions) and, in both modes, against SQLite itself: `std::db` writes 400 rows, the reader returns every cell the way SQLite's own query does.

Not covered: UTF-16 databases, un-checkpointed WAL content, index b-trees (indexes are skipped), `WITHOUT ROWID` tables, views.

## Build / test

```bash
cd packages/wj-sqlite
wj test --interpret
wj test
```
