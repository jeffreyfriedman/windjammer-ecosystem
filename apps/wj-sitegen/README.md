# wj-sitegen

Static site generator: markdown files under a source tree become HTML pages in an output directory.

No Cargo crates, no `extern fn`, no `ffi/`. Domain rendering is pure Windjammer; `std::fs` lives in adapters. Dogfoods [`wj-template`](../../packages/wj-template) for HTML escaping and page layout.

## Usage

```bash
wj-sitegen content site
```

Each `*.md` file under `content/` (recursive) is written as a matching `*.html` path under `site/` (for example `content/posts/hello.md` → `site/posts/hello.html`).

When `SITEGEN_BASE_URL` is set (absolute URL, e.g. `https://example.com`), the generator also writes:

- `sitemap.xml` — URL list for search engines
- `feed.xml` — RSS 2.0 channel with page titles and links
- `robots.txt` — crawler rules with `Sitemap:` pointing at `sitemap.xml`

Optional env overrides when argv dirs are empty:

- `SITEGEN_SRC`
- `SITEGEN_OUT`
- `SITEGEN_BASE_URL` — enable sitemap + RSS (absolute site URL)
- `SITEGEN_TITLE` — RSS channel title (defaults to `Site`)

Supported markdown (intentionally small):

- `# Heading` / `## Heading`
- paragraph lines
- HTML escaping of `&`, `<`, `>`, `"` (via `wj-template`)

The first `#` heading becomes `<title>`; otherwise the title is `Untitled`.

## Layout

```
src/
  domain/render.wj     # markdown → HTML, argv dirs
  domain/feeds.wj      # sitemap + RSS (pure)
  domain/config.wj     # SITEGEN_* env overrides
  adapters/cli_args.wj # std::env.args
  adapters/fs_site.wj  # recursive read / write
  main.wj              # composition root
tests/
  sitegen_test.wj
```

## Build / test / run

Use the **tip** compiler from the sibling `windjammer` repo (not a stale global install):

```bash
unset CARGO_TARGET_DIR
cd /path/to/windjammer && cargo build --release
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-sitegen
$WJ test
$WJ build --release src
cd build && cargo run --release -- content site
```

Example with feeds:

```bash
export SITEGEN_BASE_URL=https://example.com
export SITEGEN_TITLE="My Docs"
wj-sitegen content site
```

## License

MIT OR Apache-2.0
