# wj-sitegen

Static site generator: markdown files in a source directory become HTML pages in an output directory.

No Cargo crates, no `extern fn`, no `ffi/`. Domain rendering is pure Windjammer; `std::fs` lives in adapters.

## Usage

```bash
wj-sitegen content site
```

Each `*.md` file in `content/` (non-recursive) is written as `*.html` under `site/`.

Supported markdown (intentionally small):

- `# Heading` / `## Heading`
- paragraph lines
- HTML escaping of `&`, `<`, `>`

The first `#` heading becomes `<title>`; otherwise the title is `Untitled`.

## Layout

```
src/
  domain/render.wj     # markdown → HTML, argv dirs
  adapters/cli_args.wj # std::env.args
  adapters/fs_site.wj  # read_dir / write
  main.wj              # composition root
tests/
  sitegen_test.wj
```

## Build / test / run

```bash
unset CARGO_TARGET_DIR
export WJ=/path/to/windjammer/target/release/wj

cd apps/wj-sitegen
$WJ test
$WJ build --release src
cd build && cargo run --release -- content site
```

## License

MIT OR Apache-2.0
