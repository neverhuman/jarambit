//! jarambit-crescent — SWITCHBACK Rev C.5-anchored (C.4 until wave10) crescent genotype for the titanium hiking /
//! utility karambit (Jepson Taylor, 2026-10-11). Replaces the stick-figure planform (straight
//! grip rails + annular blade + ring) for the aesthetic lane WITHOUT relaxing any hard rule:
//! blade >= 3.375 in all three, each sharpened edge <= 4.0 in, grip 60-80 mm, ring ID 25.4 +/- 0.5,
//! double-edged + both serrated, Ti only, 0.1 mm stock, connected solid, flat brand pad,
//! holder pop-in, floors 180 / 260 / 1500 N on the detailed plate.
pub mod c4_ref;
pub mod c5_ref;
pub mod c6_ref;
pub mod geom;
pub mod genome;
pub mod pheno;
pub mod pocket_smooth;
pub mod aesthetic;
pub mod gates;
pub mod fem;
pub mod export;
pub mod qd;
pub mod record;
pub mod g1;
pub mod gci;
pub mod cad_ingest;
pub mod battery;
pub mod certification;
pub use genome::Genome;
pub use pheno::{build, Pheno};

#[cfg(test)]
mod tests;


pub mod step_export;
