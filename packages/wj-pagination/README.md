# wj-pagination

Offset and keyset pagination for list endpoints, in pure Windjammer: validate what a client asked for, compute the
SQL window, describe the page (totals, neighbours, first and last item numbers), build a compact page-number strip,
and emit an RFC 8288 `Link` header. Integer arithmetic only, with no I/O.

```windjammer
use wj_pagination::describe
use wj_pagination::link_header
use wj_pagination::page_strip
use wj_pagination::request_clamped
use wj_pagination::strip_text

fn main() {
    // raw query values from the client: ?page=7&per_page=500
    let req = request_clamped(7, 500, 20, 100)
    let page = describe(req, 1234)

    println("page ${page.page} of ${page.total_pages}, items ${page.first_item}-${page.last_item}")
    println("SQL: LIMIT ${page.per_page} OFFSET ${page.offset}")
    println(strip_text(page_strip(page.page, page.total_pages, 1)))
    println(link_header("/items?", page))
}
```

Output:

```text
page 7 of 13, items 601-700
SQL: LIMIT 100 OFFSET 600
1 ... 6 7 8 ... 13
</items?page=1>; rel="first", </items?page=6>; rel="prev", </items?page=8>; rel="next", </items?page=13>; rel="last"
```

## How it behaves

| Call | Effect |
|---|---|
| `request(page, per_page, max)` | strict validation, `Err` with a readable message |
| `request_clamped(page, per_page, default, max)` | forgiving: bad values are repaired and large pages capped |
| `describe(req, total_items)` | a `Page`; a page past the end is clamped to the last page |
| `page_count`, `offset_for`, `item_count` | the arithmetic on its own |
| `page_strip(current, total_pages, radius)` | `[1, 0, 6, 7, 8, 0, 13]` where `0` is a gap; a single hidden page is shown, never elided |
| `strip_text(strip)` | `1 ... 6 7 8 ... 13` |
| `link_header(base, page)` | `first`, `prev`, `next`, `last` relations |
| `slice_page(items, req)` | the same window applied to an in-memory list |
| `keyset_result(fetched, limit)` | fetch `limit + 1` rows, learn whether another page exists |

An empty collection has 0 pages and `first_item` / `last_item` of 0, so the result is always safe to render.

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/list_items.wj` line for line and that the
`Output` block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/pagination_test.wj
tests/docs_test.wj
examples/list_items.wj
```

## License

MIT OR Apache-2.0
