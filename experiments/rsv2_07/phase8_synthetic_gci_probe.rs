//! Arithmetic-only probe; all inputs and level identities are synthetic.
use jarambit_crescent::gci::{compute_gci, Level, MeshRef, Observable};
use serde_json::{json, Value};
use std::io::{self, Read};

fn scalar(value: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(value.as_str().unwrap(), 16).unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cases: Vec<Value> = serde_json::from_str(&input).unwrap();
    let results: Vec<Value> = cases
        .iter()
        .map(|case| {
            let values: Vec<f64> = case["values_bits"]
                .as_array()
                .unwrap()
                .iter()
                .map(scalar)
                .collect();
            let r = scalar(&case["r_bits"]);
            let fs = scalar(&case["fs_bits"]);
            let mode = case["level_mode"].as_str().unwrap();
            let mut levels: Vec<Level> = (0..3)
                .map(|i| {
                    Level::new(
                        MeshRef {
                            id: format!("synthetic-{i}"),
                            revision: "synthetic-v1".into(),
                            sha256: format!("{:064x}", i + 1),
                        },
                        [1.0, r, r * r][i],
                        Observable {
                            id: "synthetic_scalar".into(),
                            units: "dimensionless".into(),
                            region: "synthetic".into(),
                            projection: "algebra".into(),
                            metric: "scalar".into(),
                        },
                    )
                })
                .collect();
            if mode == "duplicate_content" {
                levels[1].mesh.sha256 = levels[0].mesh.sha256.clone();
            }
            let estimate = compute_gci(
                values[0],
                values[1],
                values[2],
                r,
                fs,
                if mode == "none" { None } else { Some(&levels) },
            );
            json!({
                "id": case["id"], "estimate": estimate.to_json(),
                "float_bits": {
                    "p": estimate.p.map(|v| format!("{:016x}", v.to_bits())),
                    "f_ext": estimate.f_ext.map(|v| format!("{:016x}", v.to_bits())),
                    "gci_fine": estimate.gci_fine.map(|v| format!("{:016x}", v.to_bits())),
                },
            })
        })
        .collect();
    println!("{}", serde_json::to_string(&results).unwrap());
}
