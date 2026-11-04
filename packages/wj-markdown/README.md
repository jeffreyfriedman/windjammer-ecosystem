# wj-markdown

Markdown to HTML where safe is the default. Raw HTML in the input is shown as text, `javascript:` and other script URLs never become links, and every attribute value is escaped, so rendering user-written Markdown cannot inject markup.

```windjammer
use wj_markdown::to_html

println(to_html("# Hi\n\nSome *emphasis*, a [link](https://example.com) and `code`."))
```

## What it renders

ATX and setext headings, paragraphs with hard and soft breaks, emphasis and strong (full CommonMark delimiter rules), code spans, fenced and indented code, block quotes, bullet and numbered lists (nested, tight and loose), thematic breaks, inline links, images and autolinks.

Output matches the CommonMark reference behaviour (checked against markdown-it-py) on a corpus of spec examples. Not covered: reference-style links, tables, footnotes, and decoding of named entities (well-formed entities pass through unchanged).

## Layout

```
src/lib.wj      to_html, inline_html
src/block.wj    block structure
src/inline.wj   emphasis, links, code spans
src/escape.wj   HTML and URL escaping
tests/
```

## Build / test

```bash
cd packages/wj-markdown
wj test --interpret
wj test
```
