use crate::{hash, store, Cell, Kind, Outcome, Record, Registry, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RowMeta {
    pub run_id: String,
    pub evaluation_id: String,
    pub design_id: String,
    pub group_id: String,
    pub generation: u64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub target: String,
    pub unit: String,
    pub fidelity: String,
    pub feature_schema: String,
    pub context_id: String,
    pub registry_sha256: String,
    pub csv_sha256: String,
    pub rows_sha256: String,
    pub features: Vec<String>,
    pub categorical: Vec<usize>,
    pub rows: usize,
    pub groups: usize,
    pub excluded_missing_target: usize,
    pub duplicate_designs: usize,
    pub source_receipts: Vec<String>,
}
/// Materialize immutable snapshots with a strictly last target column.
pub fn assemble(input: &Path, registry: &Registry, output: &Path) -> Result<Vec<PathBuf>> {
    registry.validate()?;
    let mut all = BTreeMap::<(String, String), Record>::new();
    let mut receipts = BTreeSet::new();
    for path in store::receipts(input)? {
        receipts.insert(hash(&std::fs::read(&path)?));
        for record in store::read_batch(&path)? {
            if record.feature_schema != registry.feature_schema {
                continue;
            }
            let key = (record.run_id.clone(), record.evaluation_id.clone());
            if let Some(previous) = all.get(&key) {
                if previous != &record {
                    return Err("conflicting evaluation identity".into());
                }
            } else {
                if all.len() >= 50_000 {
                    return Err("snapshot budget is 50,000 observations; partition the input (no silent sampling)".into());
                }
                all.insert(key, record);
            }
        }
    }
    let contexts: BTreeSet<_> = all.values().map(|r| r.context_id.clone()).collect();
    let registry_sha256 = hash(&serde_json::to_vec(registry)?);
    let mut written = Vec::new();
    for context in contexts {
        for target in &registry.targets {
            let mut csv = registry
                .features
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<_>>()
                .join(",")
                + ",target\n";
            let mut metadata = Vec::new();
            let mut missing = 0;
            let mut duplicates = 0;
            let mut seen = BTreeMap::<String, String>::new();
            for r in all.values().filter(|r| r.context_id == context) {
                let Some(Outcome::Observed { value }) = r.outcomes.get(&target.name) else {
                    missing += 1;
                    continue;
                };
                let mut fields = Vec::new();
                for feature in &registry.features {
                    let cell = r
                        .features
                        .get(&feature.name)
                        .ok_or_else(|| format!("missing required feature {}", feature.name))?;
                    fields.push(match (cell, &feature.kind) {
                        (Cell::Number(x), Kind::Numeric) => x.to_string(),
                        (Cell::Category(x), Kind::Categorical) => x.clone(),
                        (Cell::Missing, Kind::Numeric) => "NaN".into(),
                        (Cell::Missing, Kind::Categorical) => "__MISSING__".into(),
                        _ => return Err("feature type differs from registry".into()),
                    });
                }
                fields.push(value.to_string());
                let row = fields.join(",") + "\n";
                if let Some(prior) = seen.get(&r.design_id) {
                    if prior != &row {
                        return Err(format!(
                            "conflicting repeat for design {} / {}",
                            r.design_id, target.name
                        )
                        .into());
                    }
                    duplicates += 1;
                    continue;
                }
                seen.insert(r.design_id.clone(), row.clone());
                csv.push_str(&row);
                metadata.push(RowMeta {
                    run_id: r.run_id.clone(),
                    evaluation_id: r.evaluation_id.clone(),
                    design_id: r.design_id.clone(),
                    group_id: r.group_id.clone(),
                    generation: r.generation,
                });
            }
            let rows = serde_json::to_vec_pretty(&metadata)?;
            let manifest = Manifest {
                schema: 1,
                target: target.name.clone(),
                unit: target.unit.clone(),
                fidelity: target.fidelity.clone(),
                feature_schema: registry.feature_schema.clone(),
                context_id: context.clone(),
                registry_sha256: registry_sha256.clone(),
                csv_sha256: hash(csv.as_bytes()),
                rows_sha256: hash(&rows),
                features: registry.features.iter().map(|f| f.name.clone()).collect(),
                categorical: registry
                    .features
                    .iter()
                    .enumerate()
                    .filter(|(_, f)| f.kind == Kind::Categorical)
                    .map(|(i, _)| i)
                    .collect(),
                rows: metadata.len(),
                groups: metadata
                    .iter()
                    .map(|m| &m.group_id)
                    .collect::<BTreeSet<_>>()
                    .len(),
                excluded_missing_target: missing,
                duplicate_designs: duplicates,
                source_receipts: receipts.iter().cloned().collect(),
            };
            let bytes = serde_json::to_vec_pretty(&manifest)?;
            let directory = output.join(&target.name).join(&context).join(hash(&bytes));
            store::put(&directory.join("dataset.csv"), csv.as_bytes())?;
            store::put(&directory.join("rows.json"), &rows)?;
            // Publish marker last. Readers never train on an uncommitted snapshot.
            store::put(&directory.join("manifest.json"), &bytes)?;
            written.push(directory);
        }
    }
    Ok(written)
}
pub fn read_snapshot(dir: &Path) -> Result<(Manifest, Vec<Vec<String>>, Vec<RowMeta>)> {
    let m: Manifest = serde_json::from_slice(&std::fs::read(dir.join("manifest.json"))?)?;
    let feature_names: BTreeSet<_> = m.features.iter().collect();
    if feature_names.len() != m.features.len()
        || m.features
            .iter()
            .any(|f| f == "target" || !crate::safe_name(f))
        || !crate::safe_name(&m.target)
    {
        return Err("invalid/duplicate snapshot columns".into());
    }
    let csv = std::fs::read(dir.join("dataset.csv"))?;
    let row_bytes = std::fs::read(dir.join("rows.json"))?;
    if m.schema != 1 || hash(&csv) != m.csv_sha256 || hash(&row_bytes) != m.rows_sha256 {
        return Err("snapshot hash/schema mismatch".into());
    }
    let meta: Vec<RowMeta> = serde_json::from_slice(&row_bytes)?;
    let text = String::from_utf8(csv)?;
    let header = m.features.join(",") + ",target";
    let mut lines = text.lines();
    if !text.ends_with('\n') || lines.next() != Some(header.as_str()) {
        return Err("target must be the last column; header mismatch".into());
    }
    let mut rows = Vec::new();
    for line in lines {
        let fields: Vec<String> = line.split(',').map(str::to_string).collect();
        if fields.len() != m.features.len() + 1 {
            return Err("ragged dataset".into());
        }
        for (i, x) in fields.iter().enumerate() {
            if x.contains(['\t', '\r', '\n', '"']) {
                return Err("invalid field".into());
            }
            if !m.categorical.contains(&i) {
                let n: f64 = x.parse()?;
                if !(n.is_finite() || i < m.features.len() && x == "NaN") {
                    return Err("invalid numeric field or target".into());
                }
            }
        }
        rows.push(fields);
    }
    if rows.len() != m.rows
        || meta.len() != m.rows
        || m.features.len() < 3
        || m.categorical.iter().any(|&i| i >= m.features.len())
    {
        return Err("snapshot dimensions mismatch".into());
    }
    let mut ids = BTreeSet::new();
    let mut designs = BTreeSet::new();
    for row in &meta {
        if !ids.insert((&row.run_id, &row.evaluation_id)) || !designs.insert(&row.design_id) {
            return Err("duplicate snapshot row/design".into());
        }
    }
    if meta
        .iter()
        .map(|x| &x.group_id)
        .collect::<BTreeSet<_>>()
        .len()
        != m.groups
    {
        return Err("group count mismatch".into());
    }
    Ok((m, rows, meta))
}
