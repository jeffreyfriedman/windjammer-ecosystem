# Tip agent handoff — `std::config`

**Status:** Ecosystem parse path thin-wraps `std::config.parse_flat`. Tip still owns greening `config.merge` / `config.resolve` owned-HashMap call sites.

## Ask

Green remaining `bug_std_config_module_test` cases — especially `std_config_resolve_must_wire` (and `config.merge` has the same `&mut HashMap` demotion).

## API contract (`std::config`)

Format (`"toml"` | `"yaml"` / `"yml"`) is an **implementation detail** of the config process. Structured interchange stays `std::json`.

| Fn | Role |
|---|---|
| `to_json(text, format) -> Result<string, string>` | Parse → JSON text |
| `parse_flat(text, format) -> Result<HashMap<string, string>, string>` | Dotted-key flat map |
| `merge` / `overlay_matching` / `resolve` | defaults < file < matching env |
| `resolve_text(defaults, text, format, env)` | parse_flat + resolve |

Reject empty/whitespace input (yaml parity). Runtime: `windjammer_runtime::config` + `toml` crate (`parse` feature). Keep legacy `get_backend` / `BackendConfig` if present.

## Staged / partial tip work (may be incomplete or `uchg`-locked)

- `std/config.wj`
- `crates/windjammer-runtime/src/config.rs`
- `pub mod config` + `toml` dep in runtime `Cargo.toml` / `lib.rs`
- Gate: `tests/bug_std_config_module_test.rs`

If files are `uchg`-locked: `chflags nouchg` those paths before editing.

## After tip GREEN

Ecosystem: thin-wrap `packages/wj-config` over `std::config` (remove `wj-toml` path dep for parse path).

## Partial tip progress (2026-09-19)

Smoke + gate: **3/4 green** (`to_json` toml/yaml, `parse_flat`). Remaining RED:

- `std_config_resolve_must_wire` — and `config.merge` — codegen passes `&mut HashMap` into owned formals (E0308).

**Ecosystem:** `wj-config` 9/9 tip green — parse via `std::config`; merge/resolve stay package-local until tip fixes ownership.
