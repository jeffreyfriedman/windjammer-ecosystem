Run `wj build src` from this directory (a relative `src` path next to a `wj.toml`):
the build stops with `Error: Path does not exist: ` (empty path). `wj build ./src` and `wj build .` work.
Cause: `check::effects::manifest_path("src")` walks up to the empty parent and returns the relative
`wj.toml`, whose `.parent()` is the empty path; the project root passed on to `files::collect` does not exist.
Fix: treat an empty parent as `.` in `manifest_path`.
