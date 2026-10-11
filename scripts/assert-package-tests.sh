#!/usr/bin/env bash
# Fail if any apps/* or packages/* package (with wj.toml) defines zero tests. `wj test` may exit 0
# on zero tests, so this guards against silent passes. Uses `wj test --list` when the compiler
# supports it (WJ=path), otherwise greps for `fn test_*` / `@test` in the package's .wj files.
set -u
cd "$(dirname "$0")/.." || exit 2
WJ="${WJ:-}"; use_list=0
if [ -n "$WJ" ] && "$WJ" test --help 2>/dev/null | grep -q -- '--list'; then use_list=1; fi
bad=0; n=0
for dir in apps/* packages/*; do
  [ -f "$dir/wj.toml" ] || continue
  n=$((n+1)); count=0
  if [ $use_list = 1 ]; then
    count=$( (cd "$dir" && "$WJ" test --list 2>/dev/null) | grep -c . )
  else
    count=$(find "$dir" -name '*.wj' -not -path '*/build/*' -not -path '*/target/*' -print0 | \
            xargs -0 grep -hE '^[[:space:]]*(pub )?fn test_|@test' 2>/dev/null | grep -c .)
  fi
  if [ "${count:-0}" -lt 1 ]; then echo "NO TESTS: $dir"; bad=$((bad+1)); fi
done
echo "assert-package-tests: packages=$n without_tests=$bad (mode: $([ $use_list = 1 ] && echo wj-test-list || echo grep))"
[ $bad -eq 0 ]
