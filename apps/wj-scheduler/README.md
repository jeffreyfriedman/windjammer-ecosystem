# wj-scheduler

Hexagonal CLI that dogfoods [`wj-cron`](../../packages/wj-cron): check / next / explain expressions, or list / filter due entries from a crontab file.

## Commands

```bash
wj-scheduler check '@hourly' --at 2026-01-01T09:00
# → match | no-match

wj-scheduler next '*/5 * * * *' --from 2026-01-01T12:00 [--max 30]
# → 2026-01-01T12:05 | none

wj-scheduler explain '@weekly'
# → minute=0 hour=0 day_of_month=* month=* day_of_week=0

wj-scheduler list path/to/crontab
# → one expression per line (skips blanks and `#` comments)

wj-scheduler due path/to/crontab --at 2026-01-01T09:00
# → expressions matching that civil time
```

Civil times use `YYYY-MM-DDTHH:MM`. Day-of-week is computed with Sakamoto’s method (`0` = Sunday).

## Layout

```
src/
  domain/    civil time, schedule, crontab parse, argv, format
  adapters/  env argv + run_* orchestration (+ FS read for list/due)
  main.wj    composition root
tests/
  civil_test.wj
  schedule_test.wj
  crontab_test.wj
  commands_test.wj
  format_test.wj
```

## Test

```bash
# path dep requires built package first
cd ../../packages/wj-cron && unset CARGO_TARGET_DIR && wj build src
cd ../../apps/wj-scheduler
unset CARGO_TARGET_DIR
CARGO_TARGET_DIR=build/target wj test
```

## Packages dogfooded

| Package | Role |
|---|---|
| `wj-cron` | `parse_cron` (retains `source`) / `matches_cron` / `next_run` / aliases |
