//! Minimal RFC-4180 CSV line codec plus the numeric cell format shared by records and tables.
//! Pure std: compiled both inside `jarambit` and by the standalone CI engine.

/// Split one CSV line (no trailing newline) into fields. Quoted fields may contain commas and `""`.
pub fn split(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    quoted = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' && cur.is_empty() {
            quoted = true;
        } else if c == ',' {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

/// Quote a field only when it needs it.
pub fn field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn join<S: AsRef<str>>(fields: &[S]) -> String {
    fields.iter().map(|f| field(f.as_ref())).collect::<Vec<_>>().join(",")
}

/// Exact shortest round-trip representation (genes). Non-finite values are missing (empty cell).
pub fn exact(x: f64) -> String {
    if x.is_finite() {
        format!("{x}")
    } else {
        String::new()
    }
}

/// Seven significant digits, printed in shortest form (features, targets). Non-finite -> empty.
pub fn sig7(x: f64) -> String {
    if !x.is_finite() {
        return String::new();
    }
    if x == 0.0 {
        return "0".into();
    }
    let rounded: f64 = format!("{x:.6e}").parse().unwrap_or(x);
    if rounded.abs() < 1e-4 || rounded.abs() >= 1e15 {
        format!("{rounded:e}")
    } else {
        format!("{rounded}")
    }
}

/// Parse a numeric cell. Empty / NaN / null / booleans are handled; anything else is missing.
pub fn num(s: &str) -> f64 {
    let t = s.trim();
    match t {
        "" | "nan" | "NaN" | "null" | "None" | "NA" | "N/A" => f64::NAN,
        "true" | "True" | "TRUE" => 1.0,
        "false" | "False" | "FALSE" => 0.0,
        _ => t.parse::<f64>().unwrap_or(f64::NAN),
    }
}

/// True when the cell is non-empty and is not parseable as a number or boolean.
pub fn is_text(s: &str) -> bool {
    let t = s.trim();
    !t.is_empty() && num(t).is_nan() && !matches!(t, "nan" | "NaN" | "null" | "None" | "NA" | "N/A")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_handles_quotes_commas_and_empty_cells() {
        assert_eq!(split("a,,\"b,c\",\"d\"\"e\""), vec!["a", "", "b,c", "d\"e"]);
        assert_eq!(split(""), vec![""]);
        assert_eq!(split("x,"), vec!["x", ""]);
    }

    #[test]
    fn join_round_trips_through_split() {
        let row = vec!["plain".to_string(), "with,comma".into(), "q\"uote".into(), String::new()];
        assert_eq!(split(&join(&row)), row);
    }

    #[test]
    fn numeric_formats_are_compact_and_stable() {
        assert_eq!(exact(1.9133581313110282), "1.9133581313110282");
        assert_eq!(exact(f64::NAN), "");
        assert_eq!(sig7(257.123456789), "257.1235");
        assert_eq!(sig7(0.1 + 0.2), "0.3");
        assert_eq!(sig7(-1.0e-9), "-1e-9");
        assert_eq!(sig7(1.23456789e20), "1.234568e20");
        assert_eq!(num("1e-9"), 1e-9);
        assert_eq!(sig7(f64::INFINITY), "");
        assert_eq!(num("true"), 1.0);
        assert!(num("").is_nan());
        assert!(is_text("Ti-6Al-4V"));
        assert!(!is_text("3.5"));
        assert!(!is_text("NaN"));
    }
}
