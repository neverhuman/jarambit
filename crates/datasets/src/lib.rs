//! Immutable observational datasets. This crate cannot issue a certification or change search scores.
pub mod assemble;
pub mod cv;
pub mod store;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub const SCHEMA: u32 = 1;
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn safe_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 160
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_.-".contains(&c))
        && s != "."
        && s != ".."
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Cell {
    Number(f64),
    Category(String),
    Missing,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum Outcome {
    Observed { value: f64 },
    NotEvaluated,
    Failed,
    NonFinite,
}
// Parse floating tokens with Rust's correctly rounded parser without enabling
// serde_json/float_roundtrip workspace-wide (which would affect existing evidence readers).
impl<'de> Deserialize<'de> for Cell {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let text = raw.get();
        if text == "null" {
            return Ok(Self::Missing);
        }
        if text.starts_with('"') {
            return serde_json::from_str(text)
                .map(Self::Category)
                .map_err(serde::de::Error::custom);
        }
        let value: f64 = text.parse().map_err(serde::de::Error::custom)?;
        if !value.is_finite() {
            return Err(serde::de::Error::custom("nonfinite numeric feature"));
        }
        Ok(Self::Number(value))
    }
}
impl<'de> Deserialize<'de> for Outcome {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Fields {
            status: String,
            value: Option<Box<serde_json::value::RawValue>>,
        }
        let f = Fields::deserialize(d)?;
        match (f.status.as_str(), f.value) {
            ("observed", Some(raw)) => {
                let value: f64 = raw.get().parse().map_err(serde::de::Error::custom)?;
                if !value.is_finite() {
                    return Err(serde::de::Error::custom("nonfinite observed target"));
                }
                Ok(Self::Observed { value })
            }
            ("not_evaluated", None) => Ok(Self::NotEvaluated),
            ("failed", None) => Ok(Self::Failed),
            ("non_finite", None) => Ok(Self::NonFinite),
            _ => Err(serde::de::Error::custom("invalid outcome status/value")),
        }
    }
}
impl Outcome {
    pub fn measured(v: f64) -> Self {
        if v.is_finite() {
            Self::Observed { value: v }
        } else {
            Self::NonFinite
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema: u32,
    pub run_id: String,
    pub evaluation_id: String,
    pub design_id: String,
    /// Entire run or independently assigned ancestry family; never passed to the model.
    pub group_id: String,
    pub generation: u64,
    pub parents: Vec<String>,
    /// Exact evaluator/config/feature semantics, excluding run seed and generation.
    pub context_id: String,
    pub feature_schema: String,
    pub features: BTreeMap<String, Cell>,
    pub outcomes: BTreeMap<String, Outcome>,
}
impl Record {
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA
            || [
                &self.run_id,
                &self.evaluation_id,
                &self.design_id,
                &self.group_id,
                &self.context_id,
                &self.feature_schema,
            ]
            .iter()
            .any(|s| !safe_name(s))
        {
            return Err("invalid record schema or identity".into());
        }
        if self.features.len() < 3 || self.outcomes.is_empty() {
            return Err("record requires at least three features and one outcome".into());
        }
        for (name, cell) in &self.features {
            if !safe_name(name) {
                return Err("invalid feature name".into());
            }
            match cell {
                Cell::Number(v) if !v.is_finite() => {
                    return Err("nonfinite feature must be Missing".into())
                }
                Cell::Category(v) if v.is_empty() || v.contains(['\n', '\r', '\t', ',', '"']) => {
                    return Err("unsafe categorical value".into())
                }
                _ => (),
            }
        }
        for (name, outcome) in &self.outcomes {
            if !safe_name(name)
                || matches!(outcome, Outcome::Observed {value} if !value.is_finite())
            {
                return Err("invalid outcome".into());
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Numeric,
    Categorical,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    pub name: String,
    pub kind: Kind,
    pub unit: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub name: String,
    pub unit: String,
    pub fidelity: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub schema: u32,
    pub feature_schema: String,
    /// Ordered allowlist: only information available before the target evaluation.
    pub features: Vec<Feature>,
    pub targets: Vec<Target>,
}
impl Registry {
    pub fn validate(&self) -> Result<()> {
        let mut names = std::collections::BTreeSet::new();
        if self.schema != SCHEMA
            || !safe_name(&self.feature_schema)
            || self.features.len() < 3
            || self.targets.is_empty()
        {
            return Err("invalid registry".into());
        }
        for f in &self.features {
            if f.name == "target" || !safe_name(&f.name) || !names.insert(&f.name) {
                return Err("duplicate/invalid feature".into());
            }
        }
        for t in &self.targets {
            if !safe_name(&t.name) || !names.insert(&t.name) {
                return Err("target aliases a feature or another target".into());
            }
        }
        Ok(())
    }
}
