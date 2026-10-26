use windjammer_runtime::strings;
#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<Member>),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Member {
    pub key: String,
    pub value: Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]

struct Parser {
    chars: Vec<char>,
    pos: i64,
}

impl Parser {
#[inline]
fn eof(&self) -> bool {
        self.pos >= (self.chars.len() as i64)
}
#[inline]
fn peek(&self) -> char {
        self.chars[self.pos as usize]
}
#[inline]
fn skip_ws(&mut self) {
        while self.pos < ((self.chars.len() as i64)) {
            let c = self.chars[self.pos as usize];
            if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
}
#[inline]
fn fail(&self, what: &str) -> String {
        format!("JSON parse error at char {}: {}", self.pos, what)
}
fn expect_word(&mut self, word: &str) -> bool {
        let w: Vec<char> = word.chars().collect::<Vec<char>>();
        if (self.pos + (w.len() as i64)) > ((self.chars.len() as i64)) {
            return false;
        }
        let mut i: usize = 0_usize;
        while i < w.len() {
            if self.chars[self.pos + (i as i64)] != w[i] {
                return false;
            }
            i += 1;
        }
        self.pos += w.len() as i64;
        true
}
fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        let mut is_float = false;
        while self.pos < ((self.chars.len() as i64)) {
            let c = self.chars[self.pos as usize];
            if c == '.' || c == 'e' || c == 'E' {
                is_float = true;
                self.pos += 1;
            } else {
                if c == '-' || c == '+' || c >= '0' && c <= '9' {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if self.pos == start {
            return Err(self.fail("unexpected character"));
        }
        let mut text = String::new();
        let mut i = start;
        while i < self.pos {
            text = text + &self.chars[i as usize].to_string();
            i += 1;
        }
        if is_float {
            return match text.parse::<f64>() {
                Ok(x) => Ok(Json::Float(x)),
                Err(_) => { Err({ let _temp0 = format!("bad number {}", text); self.fail(&_temp0) }) },
            };
        }
        match text.parse::<i64>() {
            Ok(n) => Ok(Json::Int(n)),
            Err(_) => match text.parse::<f64>() {
                Ok(x) => Ok(Json::Float(x)),
                Err(_) => { Err({ let _temp0 = format!("bad number {}", text); self.fail(&_temp0) }) },
            },
        }
}
#[inline]
fn hex4(&mut self) -> Result<u32, String> {
        let mut code: u32 = 0_u32;
        let mut k = 0_i64;
        while k < 4_i64 {
            if self.eof() {
                return Err(self.fail("bad \\u escape"));
            }
            let c = self.chars[self.pos as usize];
            let d = hex_digit(c);
            if d < 0_i64 {
                return Err(self.fail("bad \\u escape"));
            }
            code = code * 16_u32 + d as u32;
            self.pos += 1;
            k += 1;
        }
        Ok(code)
}

fn read_string(&mut self) -> Result<String, String> {
        self.pos += 1;
        let mut out = String::new();
        while self.pos < ((self.chars.len() as i64)) {
            let c = self.chars[self.pos as usize];
            self.pos += 1;
            if c == '"' {
                return Ok(out);
            }
            if c != '\\' {
                out = out + &c.to_string();
                continue;
            }
            if self.eof() {
                break;
            }
            let e = self.chars[self.pos as usize];
            self.pos += 1;
            if e == 'n' {
                { use std::fmt::Write as _; write!(out, "{}", "\n").expect("a formatting trait implementation returned an error"); };
            } else {
                if e == 't' {
                    { use std::fmt::Write as _; write!(out, "{}", "\t").expect("a formatting trait implementation returned an error"); };
                } else {
                    if e == 'r' {
                        { use std::fmt::Write as _; write!(out, "{}", "\r").expect("a formatting trait implementation returned an error"); };
                    } else {
                        if e == 'b' {
                            { use std::fmt::Write as _; write!(out, "{}{}{}", "", chr(8_u32), "").expect("a formatting trait implementation returned an error"); };
                        } else {
                            if e == 'f' {
                                { use std::fmt::Write as _; write!(out, "{}{}{}", "", chr(12_u32), "").expect("a formatting trait implementation returned an error"); };
                            } else {
                                if e == '/' {
                                    { use std::fmt::Write as _; write!(out, "{}", "/").expect("a formatting trait implementation returned an error"); };
                                } else {
                                    if e == '"' {
                                        { use std::fmt::Write as _; write!(out, "{}", "\"").expect("a formatting trait implementation returned an error"); };
                                    } else {
                                        if e == '\\' {
                                            { use std::fmt::Write as _; write!(out, "{}", "\\").expect("a formatting trait implementation returned an error"); };
                                        } else {
                                            if e == 'u' {
                                                let mut code = self.hex4()?;
                                                if code >= 55296 && code < 56320 && self.pos + 1_i64 < ((self.chars.len() as i64)) && self.chars[self.pos as usize] == '\\' && self.chars[(self.pos as usize) + 1] == 'u' {
                                                    self.pos += 2;
                                                    let low = self.hex4()?;
                                                    code = 65536_i64 + (code - 55296_i64) * 1024_i64 + (low - 56320);
                                                }
                                                match char::from_u32(code) {
                                                    Some(ch) => {
                                                        out = out + &ch.to_string();
                                                    },
                                                    None => {
                                                        out = out + &chr(65533_u32);
                                                    },
                                                }
                                            } else {
                                                return Err(self.fail("bad escape"));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(self.fail("unterminated string"))
}

fn array(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut items = Vec::new();
        self.skip_ws();
        if !self.eof() && self.peek() == ']' {
            self.pos += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            let v = self.value()?;
            let _ = items.push(v);
            let _ = self.skip_ws();
            if self.eof() {
                return Err(self.fail("unterminated array"));
            }
            let c = self.peek();
            self.pos += 1;
            if c == ']' {
                return Ok(Json::Arr(items));
            }
            if c != ',' {
                return Err(self.fail("expected , or ]"));
            }
        }
}

fn object(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut members = Vec::new();
        self.skip_ws();
        if !self.eof() && self.peek() == '}' {
            self.pos += 1;
            return Ok(Json::Obj(members));
        }
        loop {
            let _ = self.skip_ws();
            if self.eof() || self.peek() != '"' {
                return Err(self.fail("expected a string key"));
            }
            let key = self.read_string()?;
            let _ = self.skip_ws();
            if self.eof() || self.peek() != ':' {
                return Err(self.fail("expected :"));
            }
            self.pos += 1;
            let v = self.value()?;
            let mut replaced = false;
            let mut next = Vec::new();
            for m in &members {
                if m.key == key {
                    let _ = next.push(Member { key, value: v });
                    replaced = true;
                } else {
                    let _ = next.push(m.clone());
                }
            }
            if !replaced {
                let _ = next.push(Member { key, value: v });
            }
            members = next;
            let _ = self.skip_ws();
            if self.eof() {
                return Err(self.fail("unterminated object"));
            }
            let c = self.peek();
            self.pos += 1;
            if c == '}' {
                return Ok(Json::Obj(members));
            }
            if c != ',' {
                return Err(self.fail("expected , or }"));
            }
        }
}
fn value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        if self.eof() {
            return Err(self.fail("unexpected end of input"));
        }
        let c = self.peek();
        if c == '{' {
            return self.object();
        }
        if c == '[' {
            return self.array();
        }
        if c == '"' {
            let s = self.read_string()?;
            return Ok(Json::Str(s));
        }
        if c == 't' {
            if self.expect_word("true") {
                return Ok(Json::Bool(true));
            }
            return Err(self.fail("expected true"));
        }
        if c == 'f' {
            if self.expect_word("false") {
                return Ok(Json::Bool(false));
            }
            return Err(self.fail("expected false"));
        }
        if c == 'n' {
            if self.expect_word("null") {
                return Ok(Json::Null);
            }
            return Err(self.fail("expected null"));
        }
        self.number()
}
}

#[inline]
pub fn member(key: String, value: Json) -> Member {
    Member { key, value }
}

#[inline]
pub fn object() -> Json {
    Json::Obj(Vec::new())
}

#[inline]
pub fn array() -> Json {
    Json::Arr(Vec::new())
}

#[inline]
pub fn from_str(s: String) -> Json {
    Json::Str(s)
}

#[inline]
pub fn from_int(n: i64) -> Json {
    Json::Int(n)
}

#[inline]
pub fn from_float(x: f64) -> Json {
    Json::Float(x)
}

/// The object with `key` set: replaces an existing member in place, appends a new one.
#[inline]
pub fn with(doc: Json, key: &str, value: Json) -> Json {
    match doc {
        Json::Obj(members) => {
            let mut out = Vec::new();
            let mut done = false;
            for m in members {
                if m.key == key {
                    out.push(Member { key: key.to_string(), value: value.clone() });
                    done = true;
                } else {
                    out.push(m)
                }
            }
            if !done {
                let _ = out.push(Member { key: key.to_string(), value });
            }
            Json::Obj(out)
        },
        other => other,
    }
}

/// The array with `value` appended.
#[inline]
pub fn push(doc: Json, value: Json) -> Json {
    match doc {
        Json::Arr(items) => {
            let mut out: Vec<Json> = items.clone();
            out.push(value);
            Json::Arr(out)
        },
        other => other,
    }
}

/// Member `key` of an object.
#[inline]
pub fn get(doc: Json, key: &str) -> Option<Json> {
    match doc {
        Json::Obj(members) => {
            for m in members {
                if m.key == key {
                    return Some(m.value);
                }
            }
            None
        },
        _ => None,
    }
}

/// Element `index` of an array.
#[inline]
pub fn at(doc: Json, index: i64) -> Option<Json> {
    match doc {
        Json::Arr(items) => {
            if index >= 0_i64 && index < ((items.len() as i64)) {
                return Some(items[index as usize].clone());
            }
            None
        },
        _ => None,
    }
}

/// Keys of an object, in document order.
#[inline]
pub fn keys(doc: Json) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Json::Obj(members) = doc {
        for m in members {
            let _ = out.push(m.key.clone());
        }
    }
    out
}

/// Elements of an array (empty for anything else).
#[inline]
pub fn items(doc: Json) -> Vec<Json> {
    match doc {
        Json::Arr(xs) => xs,
        _ => Vec::<Json>::new(),
    }
}

/// Members of an object, in document order (empty for anything else).
#[inline]
pub fn members(doc: Json) -> Vec<Member> {
    match doc {
        Json::Obj(ms) => ms,
        _ => Vec::<Member>::new(),
    }
}

#[inline]
pub fn as_str(doc: Json) -> Option<String> {
    match doc {
        Json::Str(s) => Some(s),
        _ => None,
    }
}

#[inline]
pub fn as_i64(doc: Json) -> Option<i64> {
    match doc {
        Json::Int(n) => Some(n),
        _ => None,
    }
}

/// Numbers read as floats, integers included.
#[inline]
pub fn as_f64(doc: Json) -> Option<f64> {
    match doc {
        Json::Float(x) => Some(x),
        Json::Int(n) => Some(n as f64),
        _ => None,
    }
}

#[inline]
pub fn as_bool(doc: Json) -> Option<bool> {
    match doc {
        Json::Bool(b) => Some::<bool>(b),
        _ => None,
    }
}

#[inline]
pub fn is_null(doc: &Json) -> bool {
    matches!(doc, Json::Null)
}

fn hex_digit(c: char) -> i64 {
    if c >= '0' && c <= '9' {
        return c as i64 - '0' as i64;
    }
    if c >= 'a' && c <= 'f' {
        return c as i64 - 'a' as i64 + 10_i64;
    }
    if c >= 'A' && c <= 'F' {
        return c as i64 - 'A' as i64 + 10_i64;
    }
    -1_i64
}

/// Parse a JSON document; object members keep document order.
#[inline]
pub fn parse(text: &str) -> Result<Json, String> {
    let chars: Vec<char> = text.chars().collect::<Vec<char>>();
    let mut p = Parser { chars, pos: 0_i64 };
    let v = p.value()?;
    p.skip_ws();
    if !p.eof() {
        return Err(p.fail("trailing characters"));
    }
    Ok(v)
}

/// JSON string literal. `ascii_only` escapes every non-ASCII character as `\uXXXX`, as Python's default does.

pub fn quote(s: &str, ascii_only: bool) -> String {
    let mut out: String = String::from("\"");
    for c in s.chars() {
        let n = c as u32;
        if c == '"' {
            { use std::fmt::Write as _; write!(out, "{}", "\\\"").expect("a formatting trait implementation returned an error"); };
        } else {
            if c == '\\' {
                { use std::fmt::Write as _; write!(out, "{}", "\\\\").expect("a formatting trait implementation returned an error"); };
            } else {
                if c == '\n' {
                    { use std::fmt::Write as _; write!(out, "{}", "\\n").expect("a formatting trait implementation returned an error"); };
                } else {
                    if c == '\r' {
                        { use std::fmt::Write as _; write!(out, "{}", "\\r").expect("a formatting trait implementation returned an error"); };
                    } else {
                        if c == '\t' {
                            { use std::fmt::Write as _; write!(out, "{}", "\\t").expect("a formatting trait implementation returned an error"); };
                        } else {
                            if n == 8_u32 {
                                { use std::fmt::Write as _; write!(out, "{}", "\\b").expect("a formatting trait implementation returned an error"); };
                            } else {
                                if n == 12_u32 {
                                    { use std::fmt::Write as _; write!(out, "{}", "\\f").expect("a formatting trait implementation returned an error"); };
                                } else {
                                    if n < 32_u32 {
                                        { use std::fmt::Write as _; write!(out, "{}{}", "\\u", hex_pad(n)).expect("a formatting trait implementation returned an error"); };
                                    } else {
                                        if n > 126_u32 && ascii_only {
                                            if n >= 65536_u32 {
                                                let v = n - 65536_u32;
                                                { use std::fmt::Write as _; write!(out, "{}{}{}{}", "\\u", hex_pad(55296_u32 + v / 1024_u32), "\\u", hex_pad(56320_u32 + v % 1024_u32)).expect("a formatting trait implementation returned an error"); };
                                            } else {
                                                { use std::fmt::Write as _; write!(out, "{}{}", "\\u", hex_pad(n)).expect("a formatting trait implementation returned an error"); };
                                            }
                                        } else {
                                            out = out + &c.to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    format!("{}{}", out, "\"")
}

#[inline]
fn hex_pad(n: u32) -> String {
    let digits: String = String::from("0123456789abcdef");
    let d: Vec<char> = digits.chars().collect::<Vec<char>>();
    let a = d[(n / 4096 % 16) as usize].to_string();
    let b = d[(n / 256 % 16) as usize].to_string();
    let c = d[(n / 16 % 16) as usize].to_string();
    let e = d[(n % 16) as usize].to_string();
    a + &b + &c + &e
}

#[inline]
fn scalar(doc: Json, ascii_only: bool) -> String {
    match doc {
        Json::Null => String::from("null"),
        Json::Bool(b) => {
            if b {
                String::from("true")
            } else {
                String::from("false")
            }
        },
        Json::Int(n) => format!("{}", n),
        Json::Float(x) => float_repr(x),
        Json::Str(s) => quote(&s, ascii_only),
        _ => String::new(),
    }
}


fn write_value(doc: Json, indent: i64, level: i64, sort_keys: bool, ascii_only: bool) -> String {
    match doc {
        Json::Arr(xs) => {
            if xs.is_empty() {
                return String::from("[]");
            }
            let mut parts: Vec<String> = Vec::new();
            for x in xs {
                parts.push(write_value(x, indent, level + 1_i64, sort_keys, ascii_only));
            }
            join_container("[", "]", &parts, indent, level, false)
        },
        Json::Obj(ms) => {
            if ms.is_empty() {
                return String::from("{}");
            }
            let mut ordered: Vec<Member> = ms.clone();
            if sort_keys {
                let _ = ordered.sort_by(|a, b| a.key.cmp(&b.key));
            }
            let mut parts: Vec<String> = Vec::new();
            for m in ordered {
                let colon: String = String::from(": ");
                parts.push(quote(&m.key, ascii_only) + &colon + &write_value(m.value.clone(), indent, level + 1_i64, sort_keys, ascii_only));
            }
            join_container("{", "}", &parts, indent, level, true)
        },
        other => scalar(other, ascii_only),
    }
}

#[inline]
fn join_container(open: &str, close: &str, parts: &Vec<String>, indent: i64, level: i64, _is_obj: bool) -> String {
    if indent <= 0_i64 {
        return open + &parts.join(", ") + &close;
    }
    let inner = strings::repeat(String::from(" "), ((level + 1_i64) * indent));
    let outer = strings::repeat(String::from(" "), (level * indent));
    format!("{}{}{}{}{}{}{}", open, "\n", inner, { let _temp0 = format!("{}{}", ",\n", inner); parts.join(&_temp0) }, "\n", outer, close)
}

/// Python's `json.dumps(doc, indent=indent, sort_keys=sort_keys)`. `indent == 0` is the single-line form
/// (`{"a": 1, "b": [1, 2]}`); `indent > 0` puts one member per line.
#[inline]
pub fn dumps(doc: Json, indent: i64, sort_keys: bool) -> String {
    write_value(doc, indent, 0_i64, sort_keys, true)
}

/// `dumps` with non-ASCII characters kept as they are (Python's `ensure_ascii=False`).
#[inline]
pub fn dumps_unicode(doc: Json, indent: i64, sort_keys: bool) -> String {
    write_value(doc, indent, 0_i64, sort_keys, false)
}

/// The character with code point `n` (U+FFFD when `n` is not a scalar value).
#[inline]
pub fn chr(n: u32) -> String {
    match char::from_u32(n) {
        Some(c) => c.to_string(),
        None => String::from("?"),
    }
}

/// Python's `repr(float)`: shortest round-trip digits, `1e+16` / `1e-05` exponent form outside
/// `1e-4 <= |x| < 1e16`, and a trailing `.0` for whole numbers.
pub fn float_repr(x: f64) -> String {
    if x != x {
        return String::from("NaN");
    }
    if x > 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64 {
        return String::from("Infinity");
    }
    if x < -179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64 {
        return String::from("-Infinity");
    }
    let neg = x < 0.0_f64 || x == 0.0_f64 && 1.0_f64 / x < 0.0_f64;
    let shown: String = {
        if neg {
            format!("{}", -x)
        } else {
            format!("{}", x)
        }
    };
    let parts = strings::split(&shown, ".");
    let int_part = (&parts[0]).to_string();
    let frac_part = {
        if parts.len() > 1_usize {
            (&parts[1]).clone()
        } else {
            String::new()
        }
    };
    let mut digits = int_part + &frac_part;
    let mut point: i64 = int_part.len() as i64;
    while digits.len() > 1_usize && digits.starts_with("0") {
        digits = (digits[1..digits.len()]).to_string();
        point -= 1;
    }
    while digits.len() > 1_usize && digits.ends_with("0") {
        digits = (digits[0..digits.len() - 1_usize]).to_string();
    }
    if digits == "0" {
        return {
            if neg {
                String::from("-0.0")
            } else {
                String::from("0.0")
            }
        };
    }
    let exp: i64 = point - 1_i64;
    let sign = {
        if neg {
            String::from("-")
        } else {
            String::new()
        }
    };
    if exp < -4_i64 || exp >= 16_i64 {
        let mantissa = {
            if digits.len() == 1_usize {
                digits
            } else {
                format!("{}{}{}", (digits[0..1]).to_string(), ".", (digits[1..digits.len()]).to_string())
            }
        };
        let e: String = {
            if exp < 0_i64 {
                String::from("-")
            } else {
                String::from("+")
            }
        };
        let mag: i64 = {
            if exp < 0_i64 {
                0_i64 - exp
            } else {
                exp
            }
        };
        let mag_text: String = {
            if mag < 10_i64 {
                format!("0{}", mag)
            } else {
                format!("{}", mag)
            }
        };
        return format!("{}{}{}{}{}", sign, mantissa, "e", e, mag_text);
    }
    let ndig: i64 = digits.len() as i64;
    if point <= 0_i64 {
        return sign + "0." + &strings::repeat(String::from("0"), (0_i64 - point)) + &digits;
    }
    if point >= ndig {
        return sign + &digits + &strings::repeat(String::from("0"), (point - ndig)) + ".0";
    }
    format!("{}{}{}{}", sign, (digits[0..point]).to_string(), ".", (digits[point as usize..digits.len()]).to_string())
}

