# wj-sql

A SQL query builder that keeps values out of the SQL text. Every value comes back as a bound parameter in placeholder
order (`?` for SQLite and MySQL, `$1`, `$2` for PostgreSQL); table and column names are validated and quoted, so a
name taken from user input cannot inject SQL. `UPDATE` and `DELETE` refuse to build without a `WHERE` unless you ask
for every row by name.

```windjammer
use wj_sql::select_from
use wj_sql::Dialect

fn main() {
    let q = select_from("users")
        .columns(["id", "name"])
        .where_eq("active", "1")
        .where_op("age", ">=", "18")
        .order_by("name", false)
        .page(2, 25)
        .build(Dialect::Dollar)
    match q {
        Ok(query) => {
            println(query.sql)
            // SELECT "id", "name" FROM "users" WHERE "active" = $1 AND "age" >= $2 ORDER BY "name" ASC LIMIT 25 OFFSET 25
            println(query.params.join(","))   // 1,18
        },
        Err(e) => println("bad query: ${e}"),
    }
}
```

Builders: `select_from`, `insert_into`, `update_table`, `delete_from`. Conditions: `where_eq`, `where_op`
(`= != < <= > >= LIKE`, `NOT LIKE`), `where_in`, `where_null`, `where_not_null`. Selects also have `join`, `order_by`,
`limit`, `offset` and `page`. Pair it with `wj-migrate` for schema changes.


## Layout

```
src/lib.wj
tests/sql_test.wj
```

## License

MIT OR Apache-2.0
