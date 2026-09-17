#!/usr/bin/env bash
# Fair release-vs-release wall clock: WJ hot path vs rust_baseline.
# Prefer tip wj with P3.350 (`wj build --release` → cargo --release).
# This script still passes --release to cargo explicitly and enables LTO.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WJ="${WJ_COMPILER:-${WJ:-}}"
if [[ -z "${WJ}" ]]; then
  if [[ -x /Users/jeffreyfriedman/src/wj/windjammer/target/release/wj ]]; then
    WJ=/Users/jeffreyfriedman/src/wj/windjammer/target/release/wj
  else
    WJ="$(command -v wj)"
  fi
fi

unset CARGO_TARGET_DIR

echo "== transpile wj-sync lib =="
(cd "$ROOT" && "$WJ" build --library --no-cargo src -o build)

echo "== transpile WJ baseline bench =="
BENCH="$ROOT/benches/wj_baseline"
(cd "$BENCH" && "$WJ" build --no-cargo src -o build)

# Tip Cargo.toml gen omits LTO; cross-crate Shared hot path needs it for ≤1.2×.
python3 - "$BENCH/build/Cargo.toml" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
t = p.read_text()
needle = "[profile.release]\nopt-level = 3\n"
repl = "[profile.release]\nopt-level = 3\nlto = true\ncodegen-units = 1\n"
if "lto" not in t and needle in t:
    p.write_text(t.replace(needle, repl))
PY

echo "== cargo --release WJ bench =="
(cd "$BENCH/build" && cargo build --release -q)
echo "== cargo --release rust baseline =="
(cd "$ROOT/benches/rust_baseline" && cargo build --release -q)

echo ""
echo "=== WJ (release) ==="
"$BENCH/build/target/release/wj-sync-wj-baseline"

echo "=== Rust (release) ==="
"$ROOT/benches/rust_baseline/target/release/wj-sync-rust-baseline"
