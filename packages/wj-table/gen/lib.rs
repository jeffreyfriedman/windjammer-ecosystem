use windjammer_runtime::strings;
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Table {
    pub headers: Vec<String>,
    pub right: Vec<bool>,
    pub rows: Vec<Vec<String>>,
}

#[inline]
pub fn new(headers: Vec<String>) -> Table {
    let mut right: Vec<bool> = Vec::new();
    for __h in &headers {
        right.push(false);
    }
    Table { headers, right, rows: Vec::new() }
}

/// Right-align the given columns (0-based): numbers read better that way.
#[inline]
pub fn right_align(t: Table, columns: &Vec<i64>) -> Table {
    let mut right: Vec<bool> = t.right;
    for c in columns.iter().copied() {
        if c >= 0_i64 && c < ((right.len() as i64)) {
            right[c as usize] = true;
        }
    }
    Table { headers: t.headers, right, rows: t.rows }
}

#[inline]
pub fn add_row(t: Table, row: Vec<String>) -> Table {
    let mut rows: Vec<Vec<String>> = t.rows;
    rows.push(row);
    Table { headers: t.headers, right: t.right, rows }
}

/// `text` padded with spaces on the left to `width` characters.
#[inline]
pub fn pad_left(text: String, width: i64) -> String {
    let n = strings::char_count(text.clone()) as i64;
    if n >= width {
        return text;
    }
    strings::repeat(String::from(" "), (width - n)) + &text
}

/// `text` padded with spaces on the right to `width` characters.
#[inline]
pub fn pad_right(text: String, width: i64) -> String {
    let n = strings::char_count(text.clone()) as i64;
    if n >= width {
        return text;
    }
    text + &strings::repeat(String::from(" "), (width - n))
}

/// Signed fixed-point text (`+1.5`, `-0.2`), like Python's `f"{x:+.1f}"`.
#[inline]
pub fn signed(x: f64, decimals: i64) -> String {
    let body = fixed(x, decimals);
    if body.starts_with("-") {
        return body;
    }
    format!("{}{}", "+", body)
}

/// A GitHub-flavoured markdown table: `| a | b |`, then `|---|---:|` (right-aligned columns end in `:`).
pub fn markdown(t: &Table) -> String {
    let mut lines: Vec<String> = Vec::new();
    { let _temp0 = format!("{}{}{}", "| ", t.headers.join(" | "), " |"); lines.push(_temp0) };
    let mut seps: Vec<String> = Vec::new();
    let mut i: usize = 0_usize;
    while i < t.headers.len() {
        let _ = seps.push({
            if t.right[i] {
                String::from("---:")
            } else {
                String::from("---")
            }
        });
        i += 1;
    }
    { let _temp0 = format!("{}{}{}", "|", seps.join("|"), "|"); lines.push(_temp0) };
    for row in &t.rows {
        { let _temp0 = format!("{}{}{}", "| ", row.join(" | "), " |"); lines.push(_temp0) };
    }
    lines.join("\n")
}

#[inline]
fn widths(t: &Table) -> Vec<i64> {
    let mut w: Vec<i64> = Vec::new();
    for h in &t.headers {
        w.push(strings::char_count(h.clone()) as i64);
    }
    for row in &t.rows {
        let mut i = 0_i32;
        while i < ((row.len() as i32)) && i < ((w.len() as i32)) {
            let n = strings::char_count(row[(i as usize)].clone()) as i64;
            if n > (w[(i as usize)]) {
                w[(i as usize)] = n;
            }
            i += 1;
        }
    }
    w
}

/// Fixed-point text with `decimals` digits (0 to 8), like Python's `f"{x:.3f}"`.
pub fn fixed(x: f64, decimals: i64) -> String {
    if decimals <= 0_i64 {
        return format!("{:.0}", x);
    }
    if decimals == 1_i64 {
        return format!("{:.1}", x);
    }
    if decimals == 2_i64 {
        return format!("{:.2}", x);
    }
    if decimals == 3_i64 {
        return format!("{:.3}", x);
    }
    if decimals == 4_i64 {
        return format!("{:.4}", x);
    }
    if decimals == 5_i64 {
        return format!("{:.5}", x);
    }
    if decimals == 6_i64 {
        return format!("{:.6}", x);
    }
    if decimals == 7_i64 {
        return format!("{:.7}", x);
    }
    format!("{:.8}", x)
}

#[inline]
fn plain_line(cells: &Vec<String>, w: &Vec<i64>, right: &Vec<bool>, gap: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i: usize = 0_usize;
    while i < w.len() {
        let cell = {
            if i < cells.len() {
                (&cells[i]).clone()
            } else {
                String::new()
            }
        };
        let _ = out.push({
            if right[i] {
                pad_left(cell.clone(), w[i])
            } else {
                pad_right(cell, w[i])
            }
        });
        i += 1;
    }
    out.join(gap)
}

/// Aligned plain text: a header line, then the rows, columns separated by `gap` (trailing spaces trimmed).
#[inline]
pub fn plain(t: Table, gap: &str) -> String {
    let w: Vec<i64> = widths(&t);
    let mut lines: Vec<String> = Vec::new();
    lines.push(strings::trim_end(&plain_line(&t.headers, &w, &t.right, gap)));
    for row in &t.rows {
        lines.push(strings::trim_end(&plain_line(row, &w, &t.right, gap)));
    }
    lines.join("\n")
}

