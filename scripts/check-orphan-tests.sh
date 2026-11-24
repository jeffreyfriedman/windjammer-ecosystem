#!/usr/bin/env bash
# Fail if any test file exists that no runner would ever execute.
#
# .wj tests  : must live under a package dir listed in scripts/wj-test-roots.txt
#              (the dirs CI runs `wj test` over).
# .rs tests  : <crate>/tests/*.rs must be discoverable by cargo: autotests not disabled,
#              or a matching [[test]] entry, or (autotests=false) listed in tests/lib.rs AND
#              hooked into the crate root (#[path = "tests/lib.rs"]).
# Exceptions : scripts/orphan-allowlist.txt, lines "path-or-dir/ | reason".
# Portable bash + git/find/grep; no builds. Run from anywhere.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT" || exit 2
ROOTS_FILE="scripts/wj-test-roots.txt"
ALLOW_FILE="scripts/orphan-allowlist.txt"
fail=0; allowed=0; checked=0

list_files() {
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then git ls-files
  else find . -type f -not -path '*/target/*' -not -path '*/.git/*' -not -path '*/node_modules/*' \
         -not -path '*/.cargo-home/*' -not -path '*/build/*' | sed 's|^\./||'; fi
}
strip() { sed -e 's/#.*//' -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' | grep -v '^$'; }

roots=(); [ -f "$ROOTS_FILE" ] && while IFS= read -r l; do roots+=("${l%/}"); done < <(strip < "$ROOTS_FILE")
allow=(); [ -f "$ALLOW_FILE" ] && while IFS= read -r l; do allow+=("$(echo "${l%%|*}" | sed 's/[[:space:]]*$//')"); done < <(strip < "$ALLOW_FILE")

is_allowed() {
  local f="$1" a
  for a in ${allow[@]+"${allow[@]}"}; do
    case "$a" in */) [[ "$f" == "$a"* ]] && return 0 ;; *) [ "$f" = "$a" ] && return 0 ;; esac
  done; return 1
}
orphan() { echo "ORPHAN: $1  ($2)"; fail=$((fail+1)); }

under_root() {
  local f="$1" r
  for r in ${roots[@]+"${roots[@]}"}; do
    [ "$r" = "." ] && return 0
    [[ "$f" == "$r"/* ]] && return 0
  done; return 1
}

# ---- stale allowlist / roots entries -------------------------------------------------
for r in ${roots[@]+"${roots[@]}"}; do
  [ -d "$r" ] || { echo "ERROR: $ROOTS_FILE lists missing dir: $r"; fail=$((fail+1)); }
done


check_rust() {
  local f="$1" tdir parent crate base stem toml
  tdir="${f%/*}"; [ "$tdir" = "$f" ] && tdir="."; base="${f##*/}"; stem="${base%.rs}"
  if [ "${tdir##*/}" != "tests" ]; then # nested: tests/sub/x.rs
    local top="$tdir" up
    while :; do up="${top%/*}"; [ "$up" = "$top" ] && break; [ "${up##*/}" = "tests" ] && break; top="$up"; done
    [ -f "$top/main.rs" ] && return 0
    [ -f "$top.rs" ] && return 0   # tests/sub.rs owns tests/sub/
    orphan "$f" "nested under tests/ with no $top.rs or $top/main.rs owner"; return 1
  fi
  parent="${tdir%/*}"; [ "$parent" = "$tdir" ] && parent="."
  if [ -f "$parent/Cargo.toml" ]; then # crate-root tests dir
    crate="$parent"
    if [ "$crate" != "${cache_crate:-}" ]; then # read per-crate files once (no forks per test file)
      cache_crate="$crate"; cache_toml="$(<"$crate/Cargo.toml")"; cache_lib=""; cache_hooked=0
      [ -f "$tdir/lib.rs" ] && cache_lib="$(<"$tdir/lib.rs")"
      local lr; for lr in "$crate/lib.rs"; do
        [ -f "$lr" ] && [[ "$(<"$lr")" == *'path = "tests/lib.rs"'* ]] && cache_hooked=1
      done
      cache_libroot_exists=0; [ -f "$crate/lib.rs" ] && cache_libroot_exists=1
    fi
    [ "$base" = "lib.rs" ] || [ "$base" = "mod.rs" ] && return 0
    local re_auto='(^|[[:space:]])autotests[[:space:]]*=[[:space:]]*false'
    [[ "$cache_toml" =~ $re_auto ]] || return 0
    [[ "$cache_toml" == *"path = \"tests/$base\""* || "$cache_toml" == *"path = \"./tests/$base\""* ]] && return 0
    local re_mod="(^|[[:space:]])mod[[:space:]]+$stem[[:space:]]*;"
    if [[ "$cache_lib" =~ $re_mod ]]; then
      [ $cache_hooked = 1 ] && return 0
      # crate-root lib.rs is generated (gitignored) and may be absent/stale: accept only when the
      # hand-owned re-hook script exists and lib.rs has not been generated yet.
      if [ -x scripts/ensure-rust-test-hook.sh ] && [ $cache_libroot_exists = 0 ]; then return 0; fi
      orphan "$f" "listed in tests/lib.rs but $crate/lib.rs lacks the tests/lib.rs hook; run scripts/ensure-rust-test-hook.sh $crate"; return 1
    fi
    orphan "$f" "autotests=false in $crate/Cargo.toml and file is neither a [[test]] nor listed in tests/lib.rs"; return 1
  fi
  # src/tests/x.rs style: parent module must declare it
  if [ "$parent" != "${cache_sparent:-}" ]; then
    cache_sparent="$parent"; cache_sok=0
    local pf re='(^|[[:space:]])(pub )?mod[[:space:]]+tests([[:space:]]|;|\{)|path[[:space:]]*=[[:space:]]*"tests/'
    for pf in "$parent"/*.rs "$parent"/*.wj; do
      [ -f "$pf" ] || continue
      [[ "$(<"$pf")" =~ $re ]] && { cache_sok=1; break; }
    done
  fi
  [ $cache_sok = 1 ] && return 0
  orphan "$f" "tests/ dir is not a crate-root tests dir and no 'mod tests' in $parent"; return 1
}

while IFS= read -r f; do
  case "$f" in
    *.wj)
      b="${f##*/}"
      case "$b" in *_test.wj|*_tests.wj|test_*.wj) ;; *) case "/$f" in */tests/*) ;; *) continue ;; esac ;; esac
      checked=$((checked+1))
      if is_allowed "$f"; then allowed=$((allowed+1)); continue; fi
      under_root "$f" || orphan "$f" "not under any dir in $ROOTS_FILE (CI never runs wj test here)"
      # CI loops over apps/* and packages/* that have a wj.toml: the owning package needs one
      case "$f" in apps/*/*|packages/*/*)
        pkg="${f%%/*}/$(echo "${f#*/}" | cut -d/ -f1)"
        [ -f "$pkg/wj.toml" ] || orphan "$f" "package $pkg has no wj.toml, so the CI loop skips it" ;;
      esac
      ;;
    *.rs)
      case "/$f" in */tests/*) ;; *) continue ;; esac
      case "$f" in */target/*|*/node_modules/*) continue ;; esac
      checked=$((checked+1))
      if is_allowed "$f"; then allowed=$((allowed+1)); continue; fi
      check_rust "$f"
      ;;
  esac
done < <(list_files | grep -E '\.(wj|rs)$' | grep -E '(_tests?\.wj$|(^|/)test_[^/]*\.wj$|(^|/)tests/)')

echo "check-orphan-tests: checked=$checked allowlisted=$allowed orphans=$fail"
[ "$fail" -eq 0 ]
