# wj-ini

## Purpose
INI configuration parsing, typed lookup, editing and serialization.

## Inputs / outputs
- In: INI text, or an `Ini` value built with `empty()` and `set`.
- Out: `Ini { sections: [Section { name, entries: [Entry { key, value }] }] }`, typed values, INI text.

## Layout
- `src/lib.wj`, `tests/ini_test.wj`, `tests/docs_test.wj`, `examples/load_config.wj`.

## Invariants
- `parse(write(x))` gives back every value, including empty, padded, quoted and multi-line ones.
- Parse errors always carry a 1-based line number.
- No I/O in the library: `[capabilities]` grants only the docs test read of the README and example.
