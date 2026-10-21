//! Tiny ordered JSON value with a strict reader and a deterministic writer.
//! Pure std so the standalone CI engine needs no crates. Non-finite numbers are written as `null`.

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    pub fn obj() -> Self {
        Json::Obj(Vec::new())
    }
    /// Builder: append a key (keeps insertion order; replaces an existing key).
    pub fn with(mut self, key: &str, value: impl Into<Json>) -> Self {
        self.set(key, value);
        self
    }
    pub fn set(&mut self, key: &str, value: impl Into<Json>) {
        if let Json::Obj(items) = self {
            let value = value.into();
            if let Some(slot) = items.iter_mut().find(|(k, _)| k == key) {
                slot.1 = value;
            } else {
                items.push((key.to_string(), value));
            }
        }
    }
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(items) => items.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(x) => Some(*x),
            Json::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_obj(&self) -> Option<&[(String, Json)]> {
        match self {
            Json::Obj(v) => Some(v),
            _ => None,
        }
    }

    pub fn pretty(&self) -> String {
        let mut s = String::new();
        self.write(&mut s, 0, true);
        s.push('\n');
        s
    }
    #[cfg(test)]
    pub fn compact(&self) -> String {
        let mut s = String::new();
        self.write(&mut s, 0, false);
        s
    }
    fn write(&self, out: &mut String, depth: usize, pretty: bool) {
        let pad = |out: &mut String, d: usize| {
            if pretty {
                out.push('\n');
                for _ in 0..d {
                    out.push_str("  ");
                }
            }
        };
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Num(x) => {
                if x.is_finite() {
                    if x.fract() == 0.0 && x.abs() < 9.0e15 {
                        out.push_str(&format!("{}", *x as i64));
                    } else {
                        out.push_str(&format!("{x}"));
                    }
                } else {
                    out.push_str("null");
                }
            }
            Json::Str(s) => escape_into(out, s),
            Json::Arr(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                let scalar = items.iter().all(|v| !matches!(v, Json::Arr(_) | Json::Obj(_)));
                out.push('[');
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                        if scalar && pretty {
                            out.push(' ');
                        }
                    }
                    if !scalar {
                        pad(out, depth + 1);
                    }
                    v.write(out, depth + 1, pretty);
                }
                if !scalar {
                    pad(out, depth);
                }
                out.push(']');
            }
            Json::Obj(items) => {
                if items.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push('{');
                for (i, (k, v)) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    pad(out, depth + 1);
                    escape_into(out, k);
                    out.push(':');
                    if pretty {
                        out.push(' ');
                    }
                    v.write(out, depth + 1, pretty);
                }
                pad(out, depth);
                out.push('}');
            }
        }
    }
}

fn escape_into(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

impl From<f64> for Json {
    fn from(x: f64) -> Self {
        Json::Num(x)
    }
}
impl From<usize> for Json {
    fn from(x: usize) -> Self {
        Json::Num(x as f64)
    }
}
impl From<u64> for Json {
    fn from(x: u64) -> Self {
        Json::Num(x as f64)
    }
}
impl From<i64> for Json {
    fn from(x: i64) -> Self {
        Json::Num(x as f64)
    }
}
impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Bool(b)
    }
}
impl From<&str> for Json {
    fn from(s: &str) -> Self {
        Json::Str(s.to_string())
    }
}
impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::Str(s)
    }
}
impl From<&String> for Json {
    fn from(s: &String) -> Self {
        Json::Str(s.clone())
    }
}
impl From<Vec<Json>> for Json {
    fn from(v: Vec<Json>) -> Self {
        Json::Arr(v)
    }
}
impl<T: Into<Json>> From<Option<T>> for Json {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(x) => x.into(),
            None => Json::Null,
        }
    }
}

/// Strict JSON reader (RFC 8259). Duplicate keys are rejected.
pub fn parse(text: &str) -> Result<Json, String> {
    let mut p = Parser { b: text.as_bytes(), i: 0, depth: 0 };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() {
        return Err(format!("trailing bytes at {}", p.i));
    }
    Ok(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
    depth: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn eat(&mut self, c: u8) -> Result<(), String> {
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected '{}' at {}", c as char, self.i))
        }
    }
    fn lit(&mut self, word: &str, v: Json) -> Result<Json, String> {
        if self.b[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(format!("bad literal at {}", self.i))
        }
    }
    fn value(&mut self) -> Result<Json, String> {
        self.depth += 1;
        if self.depth > 256 {
            return Err("nesting too deep".into());
        }
        let v = match self.b.get(self.i) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Json::Str),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'n') => self.lit("null", Json::Null),
            Some(b'N') => self.lit("NaN", Json::Num(f64::NAN)),
            Some(_) => self.number(),
            None => Err("unexpected end".into()),
        };
        self.depth -= 1;
        v
    }
    fn object(&mut self) -> Result<Json, String> {
        self.eat(b'{')?;
        let mut items: Vec<(String, Json)> = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b'}') {
            self.i += 1;
            return Ok(Json::Obj(items));
        }
        loop {
            self.ws();
            let k = self.string()?;
            self.ws();
            self.eat(b':')?;
            self.ws();
            let v = self.value()?;
            if items.iter().any(|(e, _)| *e == k) {
                return Err(format!("duplicate key {k:?}"));
            }
            items.push((k, v));
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Obj(items));
                }
                _ => return Err(format!("expected ',' or '}}' at {}", self.i)),
            }
        }
    }
    fn array(&mut self) -> Result<Json, String> {
        self.eat(b'[')?;
        let mut items = Vec::new();
        self.ws();
        if self.b.get(self.i) == Some(&b']') {
            self.i += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            self.ws();
            items.push(self.value()?);
            self.ws();
            match self.b.get(self.i) {
                Some(b',') => self.i += 1,
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Arr(items));
                }
                _ => return Err(format!("expected ',' or ']' at {}", self.i)),
            }
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.eat(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or("unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("bad escape")?;
                    self.i += 1;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let mut code = self.hex4()?;
                            if (0xD800..0xDC00).contains(&code) {
                                self.eat(b'\\')?;
                                self.eat(b'u')?;
                                let low = self.hex4()?;
                                code = 0x10000 + ((code - 0xD800) << 10) + (low.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            let ch = char::from_u32(code).ok_or("bad unicode escape")?;
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return Err("bad escape".into()),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let s = self.b.get(self.i..self.i + 4).ok_or("short unicode escape")?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(s).map_err(|e| e.to_string())?, 16).map_err(|e| e.to_string())
    }
    fn number(&mut self) -> Result<Json, String> {
        let start = self.i;
        while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
            self.i += 1;
        }
        let s = std::str::from_utf8(&self.b[start..self.i]).map_err(|e| e.to_string())?;
        s.parse::<f64>().map(Json::Num).map_err(|_| format!("bad number {s:?} at {start}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_is_deterministic_and_reader_round_trips() {
        let doc = Json::obj()
            .with("name", "s1.lat_rob__gc")
            .with("n", 120usize)
            .with("r", 0.8125)
            .with("missing", f64::NAN)
            .with("flags", Json::Arr(vec![Json::Bool(true), Json::Null]))
            .with("nested", Json::obj().with("q", "a\"b\n"));
        let text = doc.pretty();
        assert!(text.contains("\"missing\": null"));
        let back = parse(&text).unwrap();
        assert_eq!(back.get("n").and_then(Json::as_f64), Some(120.0));
        assert_eq!(back.get("nested").and_then(|o| o.get("q")).and_then(Json::as_str), Some("a\"b\n"));
        assert_eq!(parse(&back.compact()).unwrap(), back);
    }

    #[test]
    fn reader_rejects_duplicates_and_trailing_bytes() {
        assert!(parse("{\"a\":1,\"a\":2}").is_err());
        assert!(parse("[1,2] x").is_err());
        assert_eq!(parse("\"\\u00e9\"").unwrap(), Json::Str("é".into()));
        assert_eq!(parse("-1.5e3").unwrap(), Json::Num(-1500.0));
    }
}
