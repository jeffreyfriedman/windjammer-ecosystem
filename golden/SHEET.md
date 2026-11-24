# Compiled-path rules the golden programs follow

Add these to the 2 KB agent sheet; each one cost a failed compiled build in practice.

- A number into `char`: `(n as u8) as char`. A char into a number: `c as int`.
- Build strings with `out += "${piece}"`. Do not mix `out += "literal"` and `out += "${x}"` in one function; write `out = "${out}literal"` for the literal piece.
- Store `strings.trim(x)` (returns an owned string), not `x.trim()`, in a struct field.
- Name builder setters `with_method`, not `method`, when the struct has a field of that name.
- Iterator results need `.collect()`: `split`, `map`, `filter`, `keys`. Index a `Vec`, never an iterator.
- Trait method declarations (no body) say `&self`; implementations say `self`.
- Bare integer literals can default to 32 bits compiled: write `let mut total: int = 0` when the value meets `int` fields.
- Tests: `assert_eq(parse(x), Ok(v))`, or a one-line `must` helper that panics on `Err`; a test returning `Result` does not fail on `Err` interpreted.
