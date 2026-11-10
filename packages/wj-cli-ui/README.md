# wj-cli-ui

Terminal output that looks right and behaves in pipes. Colour follows `NO_COLOR` and `TERM`, widths count display columns (wide East Asian characters, combining marks, ANSI sequences), and prompts read their answers from a queue so scripts, tests and piped input all work the same way.

```windjammer
use wj_cli_ui::style::theme_from_env
use wj_cli_ui::style::green
use wj_cli_ui::progress::progress_bar

let theme = theme_from_env()
println(green(theme, "deployed"))
println(progress_bar(7, 10, 20))   // [##############------]  70%
```

## API

- `style`: `Theme`, `theme_for(no_color, term)`, `theme_from_env()`, `always()`, `never()`, `paint`, `bold`, `dim`, `red`, `green`, `yellow`, `blue`, `cyan`, `gray`, `color256`, `rgb`
- `text`: `strip_ansi`, `display_width`, `pad_left`, `pad_right`, `center`, `truncate`, `wrap`, `format_bytes`, `format_duration_ms`
- `ansi`: cursor movement, line and screen erase, alternate screen, window title, bell, `hyperlink` (OSC 8), `rewrite_line`, `clear_lines_above`
- `layout`: `boxed`, `rule`, `justify` (measured in terminal columns)
- `progress`: `progress_bar`, `styled_bar`, `spinner_frame`, `redraw`, `eta_seconds`, `rate_text`, `eta_text`, `status_line`
- `prompt`: `answers_from_stdin()`, `answers_from(lines)`, `confirm`, `ask`, `ask_int`, `ask_valid`, `select`, `multi_select`, and the pure parsers `parse_confirm`, `parse_choice`, `parse_multi`

Control sequences are the ECMA-48 / xterm values. Widths agree with Python's `unicodedata`; sizes and durations with Python's formatting. Prompts read piped standard input (`fs.read /dev/stdin`); line-by-line interactive reading is waiting for `std::io`.

## Build / test

```bash
cd packages/wj-cli-ui
wj test --interpret
wj test
```
