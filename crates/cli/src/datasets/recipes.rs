//! `recipes.conf`: which regression tables to materialize.
//!
//! Line format (`#` starts a comment):
//! ```text
//! min_rows 50                       # skip tables with fewer usable rows
//! max_rows 0                        # 0 = keep every row; otherwise a hash-ordered sample
//! leak_guard_r 0.9999               # drop features that copy the target (|r| >= this)
//! source records <dir>              # micro-record shards (relative to repo root)
//! source gen800 <dir> cap=25000     # committed gen-800 population archives
//! featureset <name> = <family.> ... # families are column prefixes, e.g. `g.` `c.`
//! targets <pattern> ... : <featureset> ...
//! skip <pattern> ...                # never used as a target
//! leak <target pattern> : <feature pattern> ...
//! ```
//! Patterns are exact column names or a prefix ending in `*` (`s1.*`, `c.mass*`).

#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Records(String),
    Gen800 { dir: String, cap: usize },
}

#[derive(Clone, Debug, Default)]
pub struct Recipes {
    pub min_rows: usize,
    pub max_rows: usize,
    pub leak_guard_r: f64,
    pub sources: Vec<Source>,
    pub featuresets: Vec<(String, Vec<String>)>,
    pub targets: Vec<(Vec<String>, Vec<String>)>,
    pub skip: Vec<String>,
    pub leaks: Vec<(String, Vec<String>)>,
}

pub fn matches(pattern: &str, name: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => pattern == name,
    }
}

impl Recipes {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut r = Recipes { min_rows: 50, max_rows: 0, leak_guard_r: 0.9999, ..Default::default() };
        for (n, raw) in text.lines().enumerate() {
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let err = |m: &str| format!("recipes.conf:{}: {m}: {raw:?}", n + 1);
            let mut words = line.split_whitespace();
            let key = words.next().unwrap_or("");
            let rest: Vec<&str> = words.collect();
            match key {
                "version" => {
                    if rest != ["1"] {
                        return Err(err("unsupported recipes version"));
                    }
                }
                "min_rows" => r.min_rows = rest.first().and_then(|v| v.parse().ok()).ok_or_else(|| err("min_rows needs an integer"))?,
                "max_rows" => r.max_rows = rest.first().and_then(|v| v.parse().ok()).ok_or_else(|| err("max_rows needs an integer"))?,
                "leak_guard_r" => {
                    r.leak_guard_r = rest.first().and_then(|v| v.parse().ok()).filter(|x: &f64| *x > 0.0 && *x <= 1.0).ok_or_else(|| err("leak_guard_r needs (0, 1]"))?
                }
                "source" => match rest.as_slice() {
                    ["records", dir] => r.sources.push(Source::Records(dir.to_string())),
                    ["gen800", dir, opts @ ..] => {
                        let mut cap = 25_000;
                        for o in opts {
                            match o.strip_prefix("cap=").and_then(|v| v.parse().ok()) {
                                Some(c) => cap = c,
                                None => return Err(err("unknown gen800 option")),
                            }
                        }
                        r.sources.push(Source::Gen800 { dir: dir.to_string(), cap });
                    }
                    _ => return Err(err("source must be `records <dir>` or `gen800 <dir> [cap=N]`")),
                },
                "featureset" => {
                    let (name, fams) = match rest.split_first() {
                        Some((name, [eq, fams @ ..])) if *eq == "=" && !fams.is_empty() => (name.to_string(), fams),
                        _ => return Err(err("featureset <name> = <family.> ...")),
                    };
                    if fams.iter().any(|f| !f.ends_with('.')) {
                        return Err(err("feature families end with '.'"));
                    }
                    if r.featuresets.iter().any(|(n, _)| *n == name) {
                        return Err(err("duplicate featureset"));
                    }
                    r.featuresets.push((name, fams.iter().map(|s| s.to_string()).collect()));
                }
                "targets" => {
                    let colon = rest.iter().position(|w| *w == ":").ok_or_else(|| err("targets <pattern>... : <featureset>..."))?;
                    let (pats, sets) = (&rest[..colon], &rest[colon + 1..]);
                    if pats.is_empty() || sets.is_empty() {
                        return Err(err("targets needs patterns and featuresets"));
                    }
                    for s in sets {
                        if !r.featuresets.iter().any(|(n, _)| n == s) {
                            return Err(err(&format!("unknown featureset {s}")));
                        }
                    }
                    r.targets.push((pats.iter().map(|s| s.to_string()).collect(), sets.iter().map(|s| s.to_string()).collect()));
                }
                "skip" => r.skip.extend(rest.iter().map(|s| s.to_string())),
                "leak" => {
                    let colon = rest.iter().position(|w| *w == ":").ok_or_else(|| err("leak <target> : <feature>..."))?;
                    if colon != 1 || rest.len() < 3 {
                        return Err(err("leak <target> : <feature>..."));
                    }
                    r.leaks.push((rest[0].to_string(), rest[2..].iter().map(|s| s.to_string()).collect()));
                }
                _ => return Err(err("unknown directive")),
            }
        }
        if r.sources.is_empty() {
            return Err("recipes.conf: no sources".into());
        }
        if r.targets.is_empty() {
            return Err("recipes.conf: no targets".into());
        }
        Ok(r)
    }

    pub fn families(&self, featureset: &str) -> &[String] {
        self.featuresets.iter().find(|(n, _)| n == featureset).map(|(_, f)| f.as_slice()).unwrap_or(&[])
    }

    /// Expand target patterns over the available columns into (target, featureset) pairs.
    pub fn expand(&self, columns: &[String]) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = Vec::new();
        for (pats, sets) in &self.targets {
            for c in columns {
                if c.starts_with("meta.") || !pats.iter().any(|p| matches(p, c)) || self.skip.iter().any(|s| matches(s, c)) {
                    continue;
                }
                for s in sets {
                    // The target column itself is always removed from its feature list (materialize).
                    let pair = (c.clone(), s.clone());
                    if !out.contains(&pair) {
                        out.push(pair);
                    }
                }
            }
        }
        out.sort();
        out
    }

    /// Feature patterns the recipe declares as leaking for `target`.
    pub fn leak_patterns(&self, target: &str) -> Vec<String> {
        self.leaks.iter().filter(|(t, _)| matches(t, target)).flat_map(|(_, f)| f.iter().cloned()).collect()
    }
}

pub fn table_name(target: &str, featureset: &str) -> String {
    let safe: String = target.chars().map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' { c } else { '_' }).collect();
    format!("{safe}__{featureset}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "version 1\nmin_rows 30 # comment\nsource records datasets/regression/records\nsource gen800 artifacts/x cap=100\n\
featureset g = g. gd.\nfeatureset gc = g. gd. c.\ntargets s1.* : g gc\ntargets h.lat_rob : g\nskip s1.nodes\nleak c.mass_g : c.blank_g c.volume*\n";

    #[test]
    fn parses_and_expands() {
        let r = Recipes::parse(SAMPLE).unwrap();
        assert_eq!(r.min_rows, 30);
        assert_eq!(r.sources[1], Source::Gen800 { dir: "artifacts/x".into(), cap: 100 });
        let cols: Vec<String> = ["meta.run_id", "s1.lat_rob", "s1.nodes", "h.lat_rob", "h.fit", "g.cp00"].iter().map(|s| s.to_string()).collect();
        let e = r.expand(&cols);
        assert_eq!(
            e,
            vec![
                ("h.lat_rob".to_string(), "g".to_string()),
                ("s1.lat_rob".to_string(), "g".to_string()),
                ("s1.lat_rob".to_string(), "gc".to_string()),
            ]
        );
        assert_eq!(r.leak_patterns("c.mass_g"), vec!["c.blank_g".to_string(), "c.volume*".to_string()]);
        assert!(r.leak_patterns("s1.lat_rob").is_empty());
        assert_eq!(table_name("s1.lat_rob", "gc"), "s1.lat_rob__gc");
    }

    #[test]
    fn rejects_malformed_lines() {
        assert!(Recipes::parse("source records x\ntargets s1.* : nope\n").is_err());
        assert!(Recipes::parse("source records x\nfeatureset g = g\n").is_err());
        assert!(Recipes::parse("source records x\nbogus 1\n").is_err());
        assert!(Recipes::parse("featureset g = g.\ntargets s1.* : g\n").is_err(), "no sources");
        assert!(matches("s1.*", "s1.lat_rob") && !matches("s1.*", "s2.x") && matches("a.b", "a.b"));
    }
}
