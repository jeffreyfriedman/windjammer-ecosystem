# wj-decimal

Fixed-point decimals for money and measurements. A `Decimal` is `units / 10^scale` held in an `int`, so `19.99` is exact, arithmetic never touches a float, and every operation that can overflow says so with an `Err`.

```windjammer
use wj_decimal::parse
use wj_decimal::mul
use wj_decimal::add
use wj_decimal::rescale
use wj_decimal::to_string
use wj_decimal::Rounding

let price = parse("19.99")?
let rate = parse("0.0825")?
let tax = rescale(mul(price, rate)?, 2, Rounding::HalfEven)?   // 1.65
println(to_string(add(price, tax)?))                           // 21.64
```

## API

- `parse`, `to_string`, `with_thousands`, `format_places`
- `add`, `sub`, `mul`, `div(a, b, scale, mode)`, `rescale(d, scale, mode)`
- `compare`, `equals` (by amount: `1.50` equals `1.5`), `signum`, `abs`, `neg`, `is_zero`, `is_negative`
- `allocate(total, ratios)` splits an amount in proportion without losing a cent: `100.00` over `[1, 1, 1]` is `33.34`, `33.33`, `33.33`
- `Rounding`: `Down`, `Up`, `Floor`, `Ceiling`, `HalfUp`, `HalfDown`, `HalfEven`

Up to 18 digits. Results are checked against Python's `decimal` module for every rounding mode.

## Layout

```
src/lib.wj
tests/decimal_test.wj   behaviour and edge cases
tests/oracle_test.wj    table of results generated with Python's decimal module
```

## Build / test

```bash
cd packages/wj-decimal
wj test --interpret
wj test
```
