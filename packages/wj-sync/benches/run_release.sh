#!/usr/bin/env bash
# Fair release-vs-release wall clock: WJ hot path vs rust_baseline.
# Requires tip wj with P3.350 (--release) and generated LTO from wj.toml profile.
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

# Assert CLI forwarded LTO (no python inject).
if ! grep -q 'lto = true' "$BENCH/build/Cargo.toml"; then
  echo "ERROR: generated Cargo.toml missing lto = true — rebuild tip wj with profile.release forwarding" >&2
  cat "$BENCH/build/Cargo.toml" >&2
  exit 1
fi

echo "== cargo --release WJ bench =="
(cd "$BENCH/build" && cargo build --release -q)
echo "== cargo --release rust baseline =="
(cd "$ROOT/benches/rust_baseline" && cargo build --release -q)

echo ""
echo "=== WJ (release) ==="
"$BENCH/build/target/release/wj-sync-wj-baseline"

echo "=== Rust (release) ==="
"$ROOT/benches/rust_baseline/target/release/wj-sync-rust-baseline"
