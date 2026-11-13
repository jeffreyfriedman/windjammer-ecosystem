# wj-money

Money for Windjammer applications, with no floating point: amounts are whole minor units (cents, pence, yen) tagged
with a currency. Mixed currencies are an error instead of a silent sum, rounding is always explicit, and `allocate`
and `split` divide an amount without losing or inventing a single cent.

```windjammer
use wj_money::Rounding
use wj_money::add
use wj_money::allocate
use wj_money::apply_bps
use wj_money::format
use wj_money::mul_int
use wj_money::parse
use wj_money::split

fn main() {
    let price = parse("19.99", "USD").unwrap()
    let subtotal = mul_int(price, 3).unwrap()
    let tax = apply_bps(subtotal, 825, Rounding::HalfUp).unwrap()
    let total = add(subtotal, tax).unwrap()
    println("subtotal ${format(subtotal)}, tax ${format(tax)}, total ${format(total)}")

    // five friends split the bill: no cent is lost
    let shares = split(total, 5).unwrap()
    println("shares: ${format(shares[0])} ${format(shares[1])} ${format(shares[2])} ${format(shares[3])} ${format(shares[4])}")

    // revenue share 70/20/10
    let parts = allocate(parse("1000.00", "USD").unwrap(), [70, 20, 10]).unwrap()
    println("revenue: ${format(parts[0])} ${format(parts[1])} ${format(parts[2])}")

    // currencies never mix by accident
    println("mixed: ${add(price, parse("5", "EUR").unwrap()).unwrap_err()}")
}
```

Output:

```text
subtotal $59.97, tax $4.95, total $64.92
shares: $12.99 $12.99 $12.98 $12.98 $12.98
revenue: $700.00 $200.00 $100.00
mixed: currency mismatch: USD and EUR
```

## How it behaves

| Call | Effect |
|---|---|
| `money(minor, currency)` | `money(1999, "USD")` is $19.99 |
| `parse(text, currency)` | `"1,234.56"`, `"-0.5"`; more decimals than the currency has is an error, never a silent rounding |
| `format`, `format_grouped`, `format_plain` | `$1,234.56`, `1,234.56`, `1234.56`; yen has 0 decimals, dinar 3 |
| `add`, `sub`, `sum`, `compare` | same currency only; overflow is an `Err` |
| `mul_int(m, quantity)` | exact multiplication |
| `apply_bps(m, bps, rounding)` | tax, tips and discounts in basis points (825 is 8.25%); `Down`, `Up`, `HalfUp`, `HalfEven` |
| `split(m, parts)`, `allocate(m, ratios)` | largest-remainder division: the shares always sum to the original amount |
| `exponent(currency)`, `symbol(currency)` | decimal places and symbol; unknown codes are an error |

Negative amounts round symmetrically about zero, so `apply_bps` of a refund mirrors the original charge.
`wj-decimal` is the companion for general fixed-point arithmetic; this package is for amounts of money.

## Verified docs

`tests/docs_test.wj` checks that the snippet above matches `examples/invoice.wj` line for line and that the `Output`
block is what the code prints. The `///` examples in `src/lib.wj` run under `wj test --interpret`.

## Layout

```
src/lib.wj
tests/money_test.wj
tests/docs_test.wj
examples/invoice.wj
```

## License

MIT OR Apache-2.0
