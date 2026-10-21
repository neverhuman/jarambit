//! Observational sidecar only: no feedback into selection, solver, or certification.
use jarambit_crescent::{genome, qd::Rec, record};
use jarambit_datasets::{
    hash, store, Cell, Feature, Kind, Outcome, Record, Registry, Result, Target,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
const FEATURE_SCHEMA: &str = "crescent_pre_solve_v1";
const CAD_NAMES: &[&str] = &[
    "mass_g",
    "blank_g",
    "stock_mm",
    "grip_mm",
    "holder_length_mm",
    "holder_width_mm",
    "holder_angle_deg",
    "min_web_mm",
    "window_count",
    "window_area_mm2",
    "perforation_count",
    "perforation_area_mm2",
];
const TARGETS: &[(&str, &str)] = &[
    ("lat20", "N"),
    ("pry20", "N"),
    ("twist20_nmm", "N_mm"),
    ("ring20", "N"),
    ("lat_rob", "N"),
    ("ring_rob", "N"),
    ("sf_hiker", "1"),
    ("sf_hunter", "1"),
    ("sf_ring", "1"),
    ("seq_ratio", "1"),
    ("twist_joint_nmm", "N_mm"),
    ("twist_sf", "1"),
    ("twist_body_joint_nmm", "N_mm"),
    ("twist_hand_joint_nmm", "N_mm"),
    ("twist_body_sf", "1"),
    ("twist_tipzone_joint_nmm", "N_mm"),
    ("twist_tipzone_sf", "1"),
    ("hm_twist_nmm", "N_mm"),
    ("hm_tip_inplane_n", "N"),
    ("hm_spine_n", "N"),
    ("hm_ringback_n", "N"),
    ("hm_twist_sf", "1"),
    ("hm_tip_inplane_sf", "1"),
    ("hm_spine_sf", "1"),
    ("hm_ringback_sf", "1"),
    ("fat_sf", "1"),
];
pub fn registry() -> Registry {
    let mut features = Vec::new();
    for i in 0..genome::NG {
        features.push(Feature {
            name: if i < genome::N_CP {
                format!("gene.cp{i:02}")
            } else {
                format!("gene.{}", genome::NAMES[i - genome::N_CP])
            },
            kind: Kind::Numeric,
            unit: "native_genome_unit".into(),
        });
    }
    features.push(Feature {
        name: "material".into(),
        kind: Kind::Categorical,
        unit: "category".into(),
    });
    for name in CAD_NAMES {
        features.push(Feature {
            name: format!("cad.{name}"),
            kind: Kind::Numeric,
            unit: if name.ends_with("_g") {
                "g"
            } else if name.ends_with("_mm2") {
                "mm2"
            } else if name.ends_with("_mm") {
                "mm"
            } else if name.ends_with("_deg") {
                "deg"
            } else {
                "count"
            }
            .into(),
        });
    }
    for i in 0..7 {
        features.push(Feature {
            name: format!("cad.removed_{i}_mm3"),
            kind: Kind::Numeric,
            unit: "mm3".into(),
        });
    }
    for i in 0..16 {
        features.push(Feature {
            name: format!("cad.descriptor_{i:02}"),
            kind: Kind::Numeric,
            unit: "1".into(),
        });
    }
    Registry {
        schema: 1,
        feature_schema: FEATURE_SCHEMA.into(),
        features,
        targets: TARGETS
            .iter()
            .map(|(name, unit)| Target {
                name: (*name).into(),
                unit: (*unit).into(),
                fidelity: "crescent_search_plate_fem".into(),
            })
            .collect(),
    }
}
fn cell(x: f64) -> Cell {
    if x.is_finite() {
        Cell::Number(x)
    } else {
        Cell::Missing
    }
}
pub fn observation(r: &Rec, run: &str, context: &str) -> Result<Record> {
    if r.g.v.len() != genome::NG || r.g.v.iter().any(|v| !v.is_finite()) {
        return Err("invalid evaluated genome".into());
    }
    let schema = registry();
    let mut features = BTreeMap::new();
    for (i, v) in r.g.v.iter().enumerate() {
        features.insert(schema.features[i].name.clone(), cell(*v));
    }
    features.insert("material".into(), Cell::Category(r.g.mat.name().into()));
    let values = [
        r.mass_g,
        r.blank_g,
        r.t,
        r.grip,
        r.holder_lw.0,
        r.holder_lw.1,
        r.holder_lw.2,
        r.min_web,
        r.n_win as f64,
        r.win_area,
        r.n_perf as f64,
        r.perf_area,
    ];
    for (name, value) in CAD_NAMES.iter().zip(values) {
        features.insert(format!("cad.{name}"), cell(value));
    }
    for (i, x) in r.removed.iter().enumerate() {
        features.insert(format!("cad.removed_{i}_mm3"), cell(*x));
    }
    for (i, x) in r.desc.iter().enumerate() {
        features.insert(format!("cad.descriptor_{i:02}"), cell(*x));
    }
    let mut outcomes = BTreeMap::new();
    if let Some(f) = &r.fem {
        let values = [
            f.lat20,
            f.pry20,
            f.twist20_nmm,
            f.ring20,
            f.lat_rob,
            f.ring_rob,
            f.sf_hiker,
            f.sf_hunter,
            f.sf_ring,
            f.seq_ratio,
            f.twist_joint_nmm,
            f.twist_sf,
            f.twist_body_joint_nmm,
            f.twist_hand_joint_nmm,
            f.twist_body_sf,
            f.twist_tipzone_joint_nmm,
            f.twist_tipzone_sf,
            f.hm_twist_nmm,
            f.hm_tip_inplane_n,
            f.hm_spine_n,
            f.hm_ringback_n,
            f.hm_twist_sf,
            f.hm_tip_inplane_sf,
            f.hm_spine_sf,
            f.hm_ringback_sf,
            f.fat_sf,
        ];
        for ((name, _), value) in TARGETS.iter().zip(values) {
            outcomes.insert((*name).into(), Outcome::measured(value));
        }
    } else {
        let status = if record::fem_status(r) == record::FemStatus::NotEvaluated {
            Outcome::NotEvaluated
        } else {
            Outcome::Failed
        };
        for (name, _) in TARGETS {
            outcomes.insert((*name).into(), status.clone());
        }
    }
    // Hash full-precision evaluated genes and material; never the rounded display CSV.
    let design_id = hash(&serde_json::to_vec(&(
        FEATURE_SCHEMA,
        &r.g.v,
        r.g.mat.name(),
    ))?);
    let record = Record {
        schema: 1,
        run_id: run.into(),
        evaluation_id: format!("g{}-e{}", r.gen, r.id),
        design_id,
        group_id: run.into(),
        generation: r.gen as u64,
        parents: r.parents.iter().map(usize::to_string).collect(),
        context_id: context.into(),
        feature_schema: FEATURE_SCHEMA.into(),
        features,
        outcomes,
    };
    record.validate()?;
    Ok(record)
}
pub struct Collector {
    root: PathBuf,
    run: String,
    context: String,
}
impl Collector {
    pub fn open(out: &Path, manifest: &jarambit_evidence::RunManifest) -> Result<Self> {
        let exe = std::env::current_exe()?;
        let producer_sha256 = hash(&std::fs::read(exe)?);
        let mut knobs = BTreeMap::new();
        for name in [
            "JARAMBIT_ANCHOR",
            "JARAMBIT_SOLVER",
            "JARAMBIT_FAER_REFINE",
            "JARAMBIT_T_MAX",
            "JARAMBIT_P1",
            "JARAMBIT_ALL_GATES",
            "JARAMBIT_SEARCH_BIAS_LAT",
            "JARAMBIT_SEARCH_BIAS_RING",
            "JARAMBIT_SEARCH_BIAS_TIPTWIST",
            "JARAMBIT_SEARCH_BIAS_SEQ",
            "JARAMBIT_TIPTWIST_GATE",
            "JARAMBIT_MIN_THROUGH",
        ] {
            knobs.insert(name, std::env::var(name).ok());
        }
        let parent_map_hash = std::env::var("JARAMBIT_P1_SFMAP")
            .ok()
            .map(|p| std::fs::read(p).map(|b| hash(&b)))
            .transpose()?;
        let context_doc = serde_json::json!({"feature_schema":FEATURE_SCHEMA,"registry_sha256":hash(&serde_json::to_vec(&registry())?),"producer_sha256":producer_sha256,"requirement_set":manifest.requirement_set,"fem_area":manifest.params.get("fem_area"),"knobs":knobs,"parent_map_sha256":parent_map_hash,"solver_backend":jarambit_physics::fine::solver_backend(),"refine_steps":jarambit_physics::fine::refine_steps(),"search_bias":jarambit_crescent::fem::search_bias(),"search_bias_extra":jarambit_crescent::fem::search_bias_extra(),"fidelity":"crescent_search_plate_fem"});
        let context = hash(&serde_json::to_vec(&context_doc)?);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos();
        let run = format!("run-{nanos}-{}", std::process::id());
        let spool = std::env::var_os("JARAMBIT_REGRESSION_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| out.join("regression"));
        let root = spool.join(&run);
        store::put(
            &root.join("context.json"),
            &serde_json::to_vec_pretty(&context_doc)?,
        )?;
        store::put(
            &root.join("run.json"),
            &serde_json::to_vec_pretty(
                &serde_json::json!({"schema":1,"run_id":run,"context_id":context,"manifest":manifest,"manifest_sha256":manifest.sha256(),"group_policy":"whole_run","qualification":"observational_search_only"}),
            )?,
        )?;
        store::put(
            &root.join("registry.json"),
            &serde_json::to_vec_pretty(&registry())?,
        )?;
        Ok(Self { root, run, context })
    }
    pub fn batch(&self, records: &[Rec]) -> std::result::Result<(), String> {
        let result = (|| -> Result<()> {
            let rows = records
                .iter()
                .map(|r| observation(r, &self.run, &self.context))
                .collect::<Result<Vec<_>>>()?;
            if !rows.is_empty() {
                store::write_batch(&self.root, &rows)?;
            }
            Ok(())
        })();
        result.map_err(|e| format!("regression collection failed: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Rec {
        Rec {
            id: 7,
            gen: 2,
            g: genome::Genome::c4_seed(5.7),
            mass_g: 80.0,
            blank_g: 90.0,
            t: 5.7,
            blade: Default::default(),
            grip: 70.0,
            aes: Default::default(),
            rejects: vec![],
            fem: None,
            spg: 0.0,
            novelty: 123.0,
            fit: 456.0,
            desc: [0.25; 16],
            holder_lw: (140.0, 50.0, 0.0),
            removed: [0.0; 7],
            min_web: 1.0,
            n_win: 0,
            win_area: 0.0,
            n_perf: 0,
            perf_area: 0.0,
            parents: vec![3, 4],
            op: "fixture",
            op_version: 1,
        }
    }
    #[test]
    fn registry_and_adapter_match_without_target_or_fitness_features() {
        let registry = registry();
        registry.validate().unwrap();
        assert_eq!(registry.features.len(), 164);
        assert_eq!(registry.targets.len(), 26);
        let r = fixture();
        let observed = observation(&r, "test-run", "test-context").unwrap();
        assert_eq!(observed.features.len(), registry.features.len());
        assert_eq!(observed.outcomes.len(), registry.targets.len());
        for f in &registry.features {
            assert!(observed.features.contains_key(&f.name));
        }
        assert!(!observed.features.contains_key("fit"));
        assert!(!observed.features.contains_key("novelty"));
        assert!(!observed.features.contains_key("lat20"));
        assert!(observed
            .outcomes
            .values()
            .all(|v| *v == Outcome::NotEvaluated));
    }
    #[test]
    fn full_precision_identity_and_missing_status_survive_collection() {
        let mut r = fixture();
        let first = observation(&r, "test-run", "test-context").unwrap();
        r.g.v[1] += 1e-12;
        assert_ne!(
            first.design_id,
            observation(&r, "test-run", "test-context")
                .unwrap()
                .design_id
        );
        r.aes.pass = true;
        let failed = observation(&r, "test-run", "test-context").unwrap();
        assert!(failed.outcomes.values().all(|v| *v == Outcome::Failed));
        let mut f = jarambit_crescent::fem::Fem::default();
        f.lat20 = f64::NAN;
        r.fem = Some(f);
        let measured = observation(&r, "test-run", "test-context").unwrap();
        assert_eq!(measured.outcomes["lat20"], Outcome::NonFinite);
        assert_eq!(measured.outcomes["pry20"], Outcome::Observed { value: 0.0 });
    }
}
