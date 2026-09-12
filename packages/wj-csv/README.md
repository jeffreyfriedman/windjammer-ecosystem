# wj-csv

CSV parse/serialize helpers for Windjammer apps.

Thin wrappers over runtime `std::csv` (RFC quoting, commas in fields).

## API

```windjammer
use wj_csv

match parse("a,b\n1,2") {
    Ok(rows) => {},
    Err(e) => println(e),
}

match parse_with_headers("id,name\n1,alice") {
    Ok(pair) => {
        let headers = pair.0
        let data = pair.1
    },
    Err(e) => println(e),
}

match write(rows) {
    Ok(text) => {},
    Err(e) => println(e),
}
```

## Layout

```
src/lib.wj
tests/csv_test.wj
```

## License

MIT OR Apache-2.0
