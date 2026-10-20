//! Missing-aware display summaries of retained arrays. No scientific evaluator is called.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) const ROSTER: &str = "docs/paper/data/test-field-roster.v1.json";
pub(crate) const CONTRACT: &str = "docs/paper/data/display-composite-contract.v1.json";
pub(crate) const REVIEW: &str = "docs/paper/data/display-composite-review.v1.json";
const ROSTER_SHA: &str = "1f1eabe810527006901ba318e167278cb154c8961894eb5f2a3deb18f5eedb01";
const CONTRACT_SHA: &str = "df34216ea111683233fadcab64d6536f52af2178da300f0ea027f8b19d9b74e9";
const REVIEW_SHA: &str = "1b4d70b8bcc4668af40427350cba12e876c972ecc1d3f0db792aec44c01b101f";

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn finite(value: &Value) -> Result<f64> {
    value
        .as_f64()
        .filter(|v| v.is_finite())
        .ok_or_else(|| "display: corrupt nonfinite or untyped scalar".into())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Observation {
    Valid(f64),
    Missing(&'static str),
}

fn observation(value: &Value, missing_reason: Option<&'static str>) -> Result<Observation> {
    if value.is_null() {
        return missing_reason
            .map(Observation::Missing)
            .ok_or_else(|| "display: unexpected null without declared missingness".into());
    }
    Ok(Observation::Valid(finite(value)?))
}

/// Equal finite values share a pooled area midrank, including signed zero ties.
pub(crate) fn weighted_midrank(values: &[Option<f64>], area: &[f64]) -> Result<Vec<Option<f64>>> {
    if values.len() != area.len() {
        return Err("display: field/area registration length differs".into());
    }
    let mut active = Vec::new();
    let mut total = 0.;
    for (i, (&value, &weight)) in values.iter().zip(area).enumerate() {
        if !weight.is_finite() || weight < 0. {
            return Err("display: invalid area weight".into());
        }
        if let Some(value) = value {
            if !value.is_finite() {
                return Err("display: corrupt nonfinite normalized input".into());
            }
            if weight > 0. {
                active.push((i, value));
                total += weight;
            }
        }
    }
    if !total.is_finite() {
        return Err("display: area denominator overflow".into());
    }
    let mut result = vec![None; values.len()];
    if total == 0. {
        return Ok(result);
    }
    active.sort_by(|a, b| a.1.total_cmp(&b.1));
    let (mut start, mut before) = (0, 0.);
    while start < active.len() {
        let mut end = start + 1;
        while end < active.len() && active[end].1 == active[start].1 {
            end += 1;
        }
        let tie_area: f64 = active[start..end].iter().map(|(i, _)| area[*i]).sum();
        let rank = (before + 0.5 * tie_area) / total;
        for &(i, _) in &active[start..end] {
            result[i] = Some(rank);
        }
        before += tie_area;
        start = end;
    }
    Ok(result)
}

pub(crate) fn mean_available(
    values: impl Iterator<Item = Option<f64>>,
) -> Result<(Option<f64>, usize)> {
    let (mut sum, mut count) = (0., 0);
    for value in values.flatten() {
        if !value.is_finite() {
            return Err("display: nonfinite contributor".into());
        }
        sum += value;
        count += 1;
    }
    if !sum.is_finite() {
        return Err("display: mean overflow".into());
    }
    Ok((
        if count == 0 {
            None
        } else {
            Some(sum / count as f64)
        },
        count,
    ))
}

pub(crate) fn weighted_quantile(
    values: &[Option<f64>],
    area: &[f64],
    p: f64,
) -> Result<Option<f64>> {
    if values.len() != area.len() || !(0. ..=1.).contains(&p) {
        return Err("display: invalid quantile registration or probability".into());
    }
    let mut active = Vec::new();
    for (&value, &weight) in values.iter().zip(area) {
        if !weight.is_finite() || weight < 0. {
            return Err("display: invalid quantile weight".into());
        }
        if let Some(value) = value {
            if !value.is_finite() {
                return Err("display: nonfinite quantile value".into());
            }
            if weight > 0. {
                active.push((value, weight));
            }
        }
    }
    active.sort_by(|a, b| a.0.total_cmp(&b.0));
    let total: f64 = active.iter().map(|v| v.1).sum();
    if !total.is_finite() {
        return Err("display: quantile area overflow".into());
    }
    let mut cumulative = 0.;
    for &(value, weight) in &active {
        cumulative += weight;
        if cumulative >= p * total {
            return Ok(Some(value));
        }
    }
    Ok(active.last().map(|v| v.0))
}

#[derive(Clone)]
pub(crate) struct Palette {
    anchors: Vec<(f64, [f64; 3])>,
    pub name: String,
}

impl Palette {
    pub(crate) fn admit(value: &Value) -> Result<Self> {
        let mut anchors = Vec::new();
        for row in value["anchors"]
            .as_array()
            .ok_or("display: missing palette anchors")?
        {
            let t = finite(&row["t"])?;
            let rgb = row["srgb"]
                .as_array()
                .ok_or("display: missing palette RGB")?;
            if rgb.len() != 3 {
                return Err("display: invalid palette dimension".into());
            }
            let rgb = [finite(&rgb[0])?, finite(&rgb[1])?, finite(&rgb[2])?];
            if !(0. ..=1.).contains(&t) || rgb.iter().any(|v| !(0. ..=255.).contains(v)) {
                return Err("display: invalid palette anchor".into());
            }
            anchors.push((t, rgb));
        }
        if anchors.len() < 2
            || anchors.first().map(|v| v.0) != Some(0.)
            || anchors.last().map(|v| v.0) != Some(1.)
            || anchors.windows(2).any(|v| v[0].0 >= v[1].0)
        {
            return Err("display: unordered palette anchors".into());
        }
        Ok(Self {
            anchors,
            name: value["name"]
                .as_str()
                .ok_or("display: missing palette name")?
                .into(),
        })
    }

    pub fn rgb(&self, value: f64) -> [u8; 3] {
        let value = value.clamp(0., 1.);
        let index = self
            .anchors
            .windows(2)
            .position(|a| value <= a[1].0)
            .unwrap_or(self.anchors.len() - 2);
        let (low, high) = (&self.anchors[index], &self.anchors[index + 1]);
        let f = (value - low.0) / (high.0 - low.0);
        std::array::from_fn(|k| ((1. - f) * low.1[k] + f * high.1[k]).round() as u8)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Contrast {
    pub low: f64,
    pub high: f64,
}

impl Contrast {
    pub fn map(self, value: f64) -> f64 {
        if self.high == self.low {
            0.5
        } else {
            ((value - self.low) / (self.high - self.low)).clamp(0., 1.)
        }
    }
}

pub(crate) struct Composite {
    pub values: Vec<Option<f64>>,
    pub counts: Vec<usize>,
    pub palette: Palette,
    pub preview: Contrast,
    pub receipt: Value,
    pub roster_table: String,
    pub input_hashes: BTreeMap<String, String>,
}

fn document(root: &Path, rel: &str, expected: &str) -> Result<Value> {
    let bytes = crate::corpus::read_bounded_file(
        &crate::corpus::resolve_safe_relpath(root, rel)?,
        32 * 1024 * 1024,
    )?;
    if hash(&bytes) != expected {
        return Err(format!("display: source binding differs for {rel}").into());
    }
    Ok(serde_json::from_slice(&bytes)?)
}

/// The reviewed roster, never a hard-coded five-map loop, controls membership.
pub(crate) fn composite(root: &Path, doc: &Value, area: &[f64]) -> Result<Composite> {
    let roster = document(root, ROSTER, ROSTER_SHA)?;
    let contract = document(root, CONTRACT, CONTRACT_SHA)?;
    let review = document(root, REVIEW, REVIEW_SHA)?;
    if review["source_contract_clearance_for_implementation"] != true
        || contract["roster"]["sha256"] != ROSTER_SHA
    {
        return Err("display: independent contract clearance or source binding differs".into());
    }
    let source_rel = contract["source"]["path"]
        .as_str()
        .ok_or("display: missing source path")?;
    let original = document(
        root,
        source_rel,
        contract["source"]["sha256"]
            .as_str()
            .ok_or("display: missing source hash")?,
    )?;
    if original != *doc
        || area.len()
            != original["mesh"]["tris"]
                .as_array()
                .ok_or("display: missing cells")?
                .len()
    {
        return Err("display: input field/mesh registration differs".into());
    }
    let field_rel = "docs/paper/field-inputs.v1.json";
    let field_sha = roster["original_input_bindings"][field_rel]["sha256"]
        .as_str()
        .ok_or("display: missing field contract hash")?;
    let field_contract = document(root, field_rel, field_sha)?;
    let (_, ring_mask) = crate::plate::source_display_masks(doc, &field_contract)?;
    let mask_ids: Vec<_> = ring_mask
        .iter()
        .enumerate()
        .filter_map(|(i, &excluded)| excluded.then_some(i))
        .collect();
    if mask_ids.len()
        != contract["channel_semantics"]["ring"]["mask"]["expected_masked_count"]
            .as_u64()
            .ok_or("display: missing mask count")? as usize
        || hash(serde_json::to_string(&mask_ids)?.as_bytes())
            != contract["channel_semantics"]["ring"]["mask"]
                ["expected_mask_ids_canonical_json_sha256"]
                .as_str()
                .ok_or("display: missing mask hash")?
    {
        return Err("display: original source mask fingerprint differs".into());
    }
    let canonical: BTreeSet<_> = contract["canonical_contributor_ids"]
        .as_array()
        .ok_or("display: missing canonical IDs")?
        .iter()
        .map(|v| v.as_str().ok_or("display: untyped canonical ID"))
        .collect::<std::result::Result<_, _>>()?;
    let rows = roster["rows"]
        .as_array()
        .ok_or("display: missing roster rows")?;
    let mut input_hashes = BTreeMap::new();
    let mut observed_documents = BTreeMap::new();
    for (path, binding) in roster["original_input_bindings"]
        .as_object()
        .ok_or("display: missing original input bindings")?
    {
        let sha = binding["sha256"]
            .as_str()
            .ok_or("display: missing original input hash")?;
        observed_documents.insert(path.clone(), document(root, path, sha)?);
        input_hashes.insert(path.clone(), sha.to_string());
    }
    for row in rows {
        if let Some(observations) = row["source_observations"].as_array() {
            for observation in observations {
                let path = observation["path"]
                    .as_str()
                    .ok_or("display: missing observation source")?;
                let sha = observation["sha256"]
                    .as_str()
                    .ok_or("display: missing observation hash")?;
                if input_hashes.get(path).is_some_and(|old| old != sha) {
                    return Err("display: conflicting observation source hashes".into());
                }
                if !observed_documents.contains_key(path) {
                    observed_documents.insert(path.to_string(), document(root, path, sha)?);
                }
                input_hashes.insert(path.to_string(), sha.to_string());
                let selector = observation["selector"]
                    .as_str()
                    .ok_or("display: missing observation selector")?;
                if observed_documents[path].pointer(selector) != Some(&observation["value"]) {
                    return Err("display: roster observation selector/value/type differs".into());
                }
            }
        }
        if row["row_kind"] == "catalog_test_or_control" {
            let selector = row["catalog_source"]["selector"]
                .as_str()
                .ok_or("display: missing catalog selector")?;
            if observed_documents["docs/tests/catalog.json"].pointer(selector)
                != Some(&row["catalog_entry_original"])
            {
                return Err("display: original catalog entry differs".into());
            }
        }
    }
    if rows.len() != 52 || canonical.len() != 4 {
        return Err("display: reviewed roster universe differs".into());
    }
    let mut seen = BTreeSet::new();
    let mut channels = Vec::new();
    let mut inventory = Vec::new();
    let mut roster_table = String::from("# 44 catalog rows + 8 original channel metadata rows; not52 independent tests.\nrow_id\tscientific_name\tobservation_stage\tquantity\tunit\tfield_availability\tcomposite_member\tspatial_missing_or_exclusion_reason\n");
    for row in rows {
        let id = row["row_id"].as_str().ok_or("display: missing row ID")?;
        let member = row["composite_member"]
            .as_bool()
            .ok_or("display: missing membership")?;
        if !seen.insert(id) || member != canonical.contains(id) {
            return Err("display: duplicate row or incompatible membership".into());
        }
        let columns = [
            id.to_string(),
            row["scientific_name"].as_str().unwrap_or("").into(),
            row["observation_stage"].as_str().unwrap_or("").into(),
            row["quantity"].as_str().unwrap_or("").into(),
            row["unit"].as_str().unwrap_or("").into(),
            row["field_availability"].as_str().unwrap_or("").into(),
            member.to_string(),
            row["composite_missing_reason"]
                .as_str()
                .unwrap_or("eligible retained response channel")
                .into(),
        ];
        roster_table.push_str(
            &columns
                .iter()
                .map(|v| {
                    v.replace(['\t', '\n', '\r'], " ")
                        .replace("source seated-region", "source half-plane")
                        .replace("source seated mask", "source half-plane mask")
                })
                .collect::<Vec<_>>()
                .join("\t"),
        );
        roster_table.push('\n');
        if !member {
            if !row["composite_missing_reason"].is_string() {
                return Err("display: excluded roster row lacks a reason".into());
            }
            inventory.push(json!({"row_id":id,"composite_member":false,"spatial_contribution":null,"reason":row["composite_missing_reason"],"role":row["role"],"alias_of":row["alias_of"],"derived_from":row["derived_from"]}));
            continue;
        }
        if row["registration"]["domain_id"] != contract["registration"]["domain_id"]
            || row["source_sha256"] != contract["source"]["sha256"]
            || row["association"] != "triangle"
            || row["individual"] != 2000
        {
            return Err("display: contributor identity/domain registration differs".into());
        }
        let selector = row["field_selector"]
            .as_str()
            .ok_or("display: missing field selector")?;
        let values = doc
            .pointer(selector)
            .and_then(Value::as_array)
            .ok_or("display: missing original field array")?;
        if values.len() != area.len() {
            return Err("display: field/mesh length mismatch".into());
        }
        let mut valid = Vec::with_capacity(area.len());
        let mut masked = Vec::new();
        let mut zero_count = 0usize;
        let mut raw_extrema = [f64::INFINITY, f64::NEG_INFINITY];
        let mut valid_area = 0.;
        for (i, value) in values.iter().enumerate() {
            let Observation::Valid(value) = observation(value, None)? else {
                return Err("display: undeclared missing field coefficient".into());
            };
            if !area[i].is_finite() || area[i] < 0. {
                return Err("display: invalid registered area".into());
            }
            raw_extrema[0] = raw_extrema[0].min(value);
            raw_extrema[1] = raw_extrema[1].max(value);
            if value == 0. {
                zero_count += 1;
            }
            let excluded = id == "retained-field:ring" && ring_mask[i];
            if excluded {
                masked.push(i);
            }
            valid.push(if excluded || area[i] == 0. {
                None
            } else {
                valid_area += area[i];
                Some(value)
            });
        }
        let normalized = weighted_midrank(&valid, area)?;
        inventory.push(json!({"row_id":id,"composite_member":true,"units":row["unit"],"selector":selector,"raw_extrema":raw_extrema,"raw_finite_zero_count":zero_count,"valid_cells":valid.iter().flatten().count(),"valid_area_mm2":valid_area,"source_masked_cell_ids":masked,"normalization":"area_weighted_pooled_tie_midrank_v1","normalized_values":normalized}));
        channels.push((id.to_string(), normalized));
    }
    if channels.len() != canonical.len() {
        return Err("display: canonical contributor missing from roster".into());
    }
    let mut values = Vec::new();
    let mut counts = Vec::new();
    let mut cells = Vec::new();
    let mut count_area = vec![0.; canonical.len() + 1];
    let mut count_cells = vec![0usize; canonical.len() + 1];
    for i in 0..area.len() {
        let (mean, count) = mean_available(channels.iter().map(|(_, v)| v[i]))?;
        let ids: Vec<_> = channels
            .iter()
            .filter_map(|(id, v)| v[i].map(|_| id.as_str()))
            .collect();
        values.push(mean);
        counts.push(count);
        count_area[count] += area[i];
        count_cells[count] += 1;
        cells.push(
            json!({"cell_id":i,"value":mean,"contributor_count":count,"contributor_test_ids":ids}),
        );
    }
    let whole_area: f64 = area.iter().sum();
    if !whole_area.is_finite() || whole_area <= 0. {
        return Err("display: empty original mesh area".into());
    }
    let low =
        weighted_quantile(&values, area, 0.05)?.ok_or("display: unavailable contrast preview")?;
    let high =
        weighted_quantile(&values, area, 0.95)?.ok_or("display: unavailable contrast preview")?;
    let mut tails = [0usize; 3];
    let mut tail_area = [0.; 3];
    for (&value, &weight) in values.iter().zip(area) {
        if let Some(value) = value {
            let bucket = if value < low {
                0
            } else if value > high {
                2
            } else {
                1
            };
            tails[bucket] += 1;
            tail_area[bucket] += weight;
        }
    }
    let palette = Palette::admit(&contract["palette"])?;
    let fractions: Vec<_> = count_area.iter().map(|a| a / whole_area).collect();
    let receipt = json!({
        "schema":"jarambit.display_composite_projection.v1",
        "roster":{"path":ROSTER,"sha256":ROSTER_SHA},"contract":{"path":CONTRACT,"sha256":CONTRACT_SHA},
        "independent_contract_review":{"path":REVIEW,"sha256":REVIEW_SHA,"scope":"implementation semantics only; artifact and final acceptance separate"},
        "source":contract["source"],"producer_revision":contract["producer_revision"],"subject":contract["subject"],
        "active_authority":"reviewed display-composite-contract.v1.json and exact independent contract review; original figure-semantics hero selector is historical",
        "supersedes":{"original_contract":"docs/paper/figure-semantics.v1.json","selector":"/hero_composite","old_published_hero_sha256":"7cfbfe6b1dbe275838ff59b09a8b12239d193b3d835e24c6b6934eceb2cc6dfe","old_recipe":"five maps including dependent envelope; source-order ties; candidate-minmax after mean; q5/4/3","preservation":"Original source contracts and prior artifacts retained unchanged in source history; not active validation authority for this new recipe"},
        "inventory_metadata_rows":rows.len(),"catalog_rows":44,"original_channel_addenda":8,"eligible_registered_response_channels":channels.len(),
        "test_independence_claim":false,"scientific_fitness":false,"secondary_minmax_after_mean":false,
        "whole_source_area_mm2":whole_area,"support_counts":count_cells,"support_area_mm2":count_area,"support_area_fractions":fractions,
        "support_denominator":"4 eligible response channels; area fractions over all positive original source-triangle area, before overlays/projection",
        "rows":inventory,"cells":cells,"palette":contract["palette"],"surface_lighting":"none",
        "default_contrast":{"domain":[0.,1.],"status":"full-range; actual readability review pending"},
        "Q05_Q95_preview":{"low":low,"high":high,"counts_below_inside_above":tails,"area_mm2_below_inside_above":tail_area,"whole_source_area_mm2":whole_area,"status":"additional preview; not selected without independent readability review"},
        "display_transfer":contract["display_transfer"],"mask_reconstruction":"original source-bound plate admission; no value==0 missingness heuristic",
        "current_mask_terminology":{"contract":"docs/paper/data/reporting-interpretation-contract.v2.json","ring":"source-masked half-plane centroid.x >= sx0 - 3.0; includes seated and unloaded regions","original_contract_and_mask_bytes_preserved":true},
        "scope":"Reporting only; no new solve/CAD/search, scalar broadcast or foreign-subject vector. Surface color is display transfer of retained plate response, not volumetric stress."
    });
    for (rel, sha) in [
        (ROSTER, ROSTER_SHA),
        (CONTRACT, CONTRACT_SHA),
        (REVIEW, REVIEW_SHA),
        (field_rel, field_sha),
    ] {
        input_hashes.insert(rel.to_string(), sha.to_string());
    }
    input_hashes.insert(
        source_rel.to_string(),
        contract["source"]["sha256"].as_str().unwrap().into(),
    );
    Ok(Composite {
        values,
        counts,
        palette,
        preview: Contrast { low, high },
        receipt,
        roster_table,
        input_hashes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pooled_ties_preserve_zeros_constants_and_source_order_independence() {
        let values = [Some(0.), Some(-0.), Some(2.), Some(2.), None];
        assert_eq!(
            weighted_midrank(&values, &[1., 3., 2., 2., 7.]).unwrap(),
            [Some(0.25), Some(0.25), Some(0.75), Some(0.75), None]
        );
        assert_eq!(
            weighted_midrank(&[Some(9.), Some(9.)], &[1., 8.]).unwrap(),
            [Some(0.5), Some(0.5)]
        );
        assert_eq!(
            weighted_midrank(&[Some(0.), None], &[0., 1.]).unwrap(),
            [None, None]
        );
        assert!(weighted_midrank(&[Some(f64::INFINITY)], &[1.]).is_err());
        assert!(weighted_midrank(&[Some(1.)], &[1., 2.]).is_err());
        assert!(weighted_midrank(&[Some(1.)], &[-1.]).is_err());
    }

    #[test]
    fn missing_mean_is_typed_and_missing_rows_have_no_numerical_weight() {
        assert_eq!(mean_available([None, None].into_iter()).unwrap(), (None, 0));
        assert_eq!(
            mean_available([Some(0.), None].into_iter()).unwrap(),
            (Some(0.), 1)
        );
        assert_eq!(
            mean_available([Some(0.2), None, Some(0.8)].into_iter()).unwrap(),
            (Some(0.5), 2)
        );
        assert!(mean_available([Some(f64::NAN)].into_iter()).is_err());
        assert_eq!(
            observation(&Value::Null, Some("scalar_only")).unwrap(),
            Observation::Missing("scalar_only")
        );
        assert!(observation(&Value::Null, None).is_err());
        assert!(observation(&json!("NaN"), None).is_err());
        assert_eq!(
            observation(&json!(0.), None).unwrap(),
            Observation::Valid(0.)
        );
    }

    #[test]
    fn unequal_source_masks_expose_cell_support_without_scalar_broadcast() {
        let area = [1., 1., 1.];
        let a = weighted_midrank(&[Some(0.), Some(10.), None], &area).unwrap();
        let b = weighted_midrank(&[Some(1.), None, Some(1.)], &area).unwrap();
        let actual: Vec<_> = (0..3)
            .map(|i| mean_available([a[i], b[i], None].into_iter()).unwrap())
            .collect();
        assert_eq!(actual, [(Some(0.375), 2), (Some(0.75), 1), (Some(0.5), 1)]);
    }

    #[test]
    fn quantile_ties_and_contrast_keep_full_range_mean_distinct() {
        let values = [Some(0.), Some(0.5), Some(0.5), Some(1.)];
        assert_eq!(
            weighted_quantile(&values, &[1., 2., 3., 1.], 0.5).unwrap(),
            Some(0.5)
        );
        assert_eq!(weighted_quantile(&[None], &[1.], 0.5).unwrap(), None);
        assert_eq!(
            Contrast {
                low: 0.2,
                high: 0.8
            }
            .map(0.1),
            0.
        );
        assert_eq!(
            Contrast {
                low: 0.2,
                high: 0.8
            }
            .map(0.9),
            1.
        );
        assert_eq!(
            Contrast {
                low: 0.5,
                high: 0.5
            }
            .map(0.5),
            0.5
        );
    }

    #[test]
    fn shared_palette_has_literal_reviewed_endpoints() {
        let palette = Palette::admit(&json!({"name":"neutral","anchors":[{"t":0.,"srgb":[8,28,74]},{"t":0.5,"srgb":[250,204,21]},{"t":1.,"srgb":[153,27,27]}]})).unwrap();
        assert_eq!(palette.rgb(0.), [8, 28, 74]);
        assert_eq!(palette.rgb(0.5), [250, 204, 21]);
        assert_eq!(palette.rgb(1.), [153, 27, 27]);
    }
}
