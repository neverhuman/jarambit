//! Display-only projection of every admitted numeric observation, separated by source scope.
//! The caller admits original identities and supplies complete original extrema, never quantiles.
use crate::population_story::{PositiveAxis, Preference};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Write as _;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const EPSILON: f64 = 1e-6;
pub const WIDTH: usize = 951;
pub const HEIGHT: usize = 421;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Abscissa {
    Generation {
        last: u64,
    },
    /// Already one-based; no extra shift and no invented generation identity.
    FileOrdinal {
        last: u64,
    },
}
impl Abscissa {
    fn bounds(self) -> [u64; 2] {
        match self {
            Self::Generation { last } => [0, last],
            Self::FileOrdinal { last } => [1, last],
        }
    }
    fn coordinate(self, original: u64) -> Result<f64> {
        let [lo, hi] = self.bounds();
        if original < lo || original > hi || original >= (1_u64 << 53) {
            return Err("positive points: x outside exact original integer scope".into());
        }
        Ok(match self {
            Self::Generation { .. } => (original as f64 + 1.).log10(),
            Self::FileOrdinal { .. } => (original as f64).log10(),
        })
    }
    fn metadata(self) -> Value {
        json!({"original_bounds":self.bounds(), "logarithm_base":10,
            "coordinate_formula":match self {Self::Generation {..}=>"log10(generation+1)",Self::FileOrdinal {..}=>"log10(one_based_file_ordinal)"},
            "inverse_formula":match self {Self::Generation {..}=>"10^x-1",Self::FileOrdinal {..}=>"10^x"}})
    }
}

pub struct PositivePointRaster {
    axis: PositiveAxis,
    original_domain: [f64; 2],
    preference: Preference,
    gap: bool,
    x_axis: Abscissa,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    expected: u64,
    operations: u64,
    overlaps: u64,
    negatives: u64,
    zeros: u64,
    observed: [f64; 2],
    centers: Vec<u8>,
    ink: Vec<u8>,
    identity_hash: Sha256,
    operation_hash: Sha256,
    best_by_x: BTreeMap<u64, f64>,
    leader_ties: u64,
    first_leader_x: Option<u64>,
}
impl PositivePointRaster {
    pub fn new(
        domain: [f64; 2],
        preference: Preference,
        gap: bool,
        x_axis: Abscissa,
        expected: u64,
    ) -> Result<Self> {
        if expected == 0 {
            return Err("positive points: empty numeric scope".into());
        }
        let axis = PositiveAxis::new(domain, preference, gap, EPSILON)?;
        let [x0, x1] = x_axis.bounds();
        let x_bounds = [x_axis.coordinate(x0)?, x_axis.coordinate(x1)?];
        if x_bounds[0] > x_bounds[1] {
            return Err("positive points: reversed x domain".into());
        }
        let a = axis.transform(domain[0])?;
        let b = axis.transform(domain[1])?;
        Ok(Self {
            axis,
            original_domain: domain,
            preference,
            gap,
            x_axis,
            x_bounds,
            y_bounds: [a.min(b), a.max(b)],
            expected,
            operations: 0,
            overlaps: 0,
            negatives: 0,
            zeros: 0,
            observed: [f64::INFINITY, f64::NEG_INFINITY],
            centers: vec![0; WIDTH * HEIGHT],
            ink: vec![0; WIDTH * HEIGHT],
            identity_hash: Sha256::new(),
            operation_hash: Sha256::new(),
            best_by_x: BTreeMap::new(),
            leader_ties: 0,
            first_leader_x: None,
        })
    }
    /// One call per original observation, including overlapping marks and repeated values.
    /// Identity is the caller's original, fixed-width LE u64 tuple (not a display-selected ID).
    pub fn paint(&mut self, identity: &[u64], x: u64, original: f64) -> Result<[usize; 2]> {
        if identity.is_empty() || self.operations >= self.expected {
            return Err("positive points: missing identity or extra observation".into());
        }
        let [px, py] = self.position(x, original)?;
        let offset = py * WIDTH + px;
        self.overlaps += u64::from(self.centers[offset] != 0);
        self.centers[offset] = 1;
        // Fixed 3x3 high-contrast point footprint; no density, jitter, or class replacement.
        // Centers at exact full-range endpoints remain included. Only the mark footprint is bounded.
        for iy in py.saturating_sub(1)..=(py + 1).min(HEIGHT - 1) {
            for ix in px.saturating_sub(1)..=(px + 1).min(WIDTH - 1) {
                self.ink[iy * WIDTH + ix] = 1;
            }
        }
        self.operations += 1;
        self.negatives += u64::from(original < 0.);
        self.zeros += u64::from(original == 0.);
        self.observed[0] = self.observed[0].min(original);
        self.observed[1] = self.observed[1].max(original);
        for &id in identity {
            self.identity_hash.update(id.to_le_bytes());
        }
        self.operation_hash
            .update((identity.len() as u64).to_le_bytes());
        for &id in identity {
            self.operation_hash.update(id.to_le_bytes());
        }
        for part in [
            x.to_le_bytes(),
            original.to_bits().to_le_bytes(),
            (px as u64).to_le_bytes(),
            (py as u64).to_le_bytes(),
        ] {
            self.operation_hash.update(part);
        }
        let high = self.preference == Preference::Higher;
        self.best_by_x
            .entry(x)
            .and_modify(|best| {
                if (high && original > *best) || (!high && original < *best) {
                    *best = original;
                }
            })
            .or_insert(original);
        let leader = if high {
            self.original_domain[1]
        } else {
            self.original_domain[0]
        };
        if original == leader {
            self.leader_ties += 1;
            self.first_leader_x = Some(self.first_leader_x.map_or(x, |old| old.min(x)));
        }
        Ok([px, py])
    }
    /// Same mapping for every raw point and the derived cumulative-best overlay.
    pub fn position(&self, x: u64, original: f64) -> Result<[usize; 2]> {
        let xx = self.x_axis.coordinate(x)?;
        let yy = self.axis.transform(original)?;
        let fraction = |v: f64, b: [f64; 2]| {
            if b[0] == b[1] {
                0.5
            } else {
                (v - b[0]) / (b[1] - b[0])
            }
        };
        let fx = fraction(xx, self.x_bounds);
        let fy = fraction(yy, self.y_bounds);
        if !fx.is_finite()
            || !fy.is_finite()
            || !(0. ..=1.).contains(&fx)
            || !(0. ..=1.).contains(&fy)
        {
            return Err("positive points: projection outside complete domain; no clipping".into());
        }
        Ok([
            (fx * (WIDTH - 1) as f64).round() as usize,
            ((1. - fy) * (HEIGHT - 1) as f64).round() as usize,
        ])
    }
    pub fn pixels(&self) -> &[u8] {
        &self.ink
    }
    pub fn identity_order_sha256(&self) -> String {
        format!("{:x}", self.identity_hash.clone().finalize())
    }
    pub fn receipt(&self) -> Result<Value> {
        if self.operations != self.expected
            || self.observed != self.original_domain
            || self.leader_ties == 0
        {
            return Err("positive points: incomplete operations/extrema/leader census".into());
        }
        // Bind the original in-memory parameters before any JSON parse can round a float.
        let axis = self.axis.metadata();
        let axis_canonical_json_sha256 =
            format!("{:x}", Sha256::digest(serde_json::to_vec(&axis)?));
        Ok(
            json!({"schema":"jarambit.positive_all_record_projection.v1", "original_domain":self.original_domain,
            "axis":axis,"axis_canonical_json_sha256":axis_canonical_json_sha256,"x_axis":self.x_axis.metadata(),"original_negative_records":self.negatives,"original_zero_records":self.zeros,
            "numeric_point_operations":self.operations,"expected_numeric_records":self.expected,"excluded_numeric_records":0,
            "overlapping_center_operations":self.overlaps,"occupied_centers":self.centers.iter().filter(|&&v|v!=0).count(),
            "identity_order_sha256":self.identity_order_sha256(),"identity_hash_encoding":"ordered caller-specified fixed-width LE u64 original identity tuple",
            "point_operation_sha256":format!("{:x}",self.operation_hash.clone().finalize()),
            "point_operation_hash_encoding":"LE u64 identity tuple length, identity tuple, original x, original f64 bits, raster x, raster y",
            "raster":{"width":WIDTH,"height":HEIGHT,"center_mask_sha256":format!("{:x}",Sha256::digest(&self.centers)),
                "visible_mask_sha256":format!("{:x}",Sha256::digest(&self.ink)),"point_footprint":"fixed 3x3 pixels, bounded footprint at panel edge; every original center included", "density":false},
            "leader_ties":self.leader_ties,
            "first_retained_leader_generation":if matches!(self.x_axis,Abscissa::Generation {..}) {self.first_leader_x} else {None},
            "minimum_file_ordinal_among_ties":if matches!(self.x_axis,Abscissa::FileOrdinal {..}) {self.first_leader_x} else {None},
            "overlay":if matches!(self.x_axis,Abscissa::Generation {..}) {"within-scope cumulative best derived from exactly these raw observations; no median invented; gaps not bridged"} else {"none; file accession order is not a proven campaign/time sequence"},
            "source_membership_admitted_by":"caller, before display projection", "new_fitness_or_geometry":false,
            "physical_ratio_meaning":false,"full_range_asinh_companion_required":true}),
        )
    }
    pub fn render(&self, png_b64: &str, title: &str) -> Result<(String, Value)> {
        let mut receipt = self.receipt()?;
        let escape = |s: &str| {
            s.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
        };
        if title.chars().count() > 92 || png_b64.is_empty() {
            return Err("positive points: invalid panel title or PNG".into());
        }
        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="850" viewBox="0 0 1200 850" font-family="Inter, sans-serif"><rect width="1200" height="850" fill="#ffffff"/><text x="30" y="40" font-size="27" font-weight="700" fill="#0f172a">{}</text><text x="30" y="77" font-size="23" fill="#334155">{} original observations; every numeric row painted, including overlaps.</text><image x="200" y="150" width="951" height="421" preserveAspectRatio="none" image-rendering="pixelated" href="data:image/png;base64,{png_b64}"/>"##,
            escape(title),
            self.operations
        );
        let mut tick_rows = Vec::new();
        let fractions: &[f64] = if self.original_domain[0] == self.original_domain[1] {
            &[0.5]
        } else {
            &[0., 0.25, 0.5, 0.75, 1.]
        };
        for &fraction in fractions {
            let t = if fraction == 0. {
                self.y_bounds[0]
            } else if fraction == 1. {
                self.y_bounds[1]
            } else {
                self.y_bounds[0] + fraction * (self.y_bounds[1] - self.y_bounds[0])
            };
            let raw = self.axis.inverse_log(t)?;
            let y = 150. + 420. * (1. - fraction);
            let label = if raw == 0. {
                "0".to_string()
            } else {
                format!("{raw:.6e}")
            };
            write!(
                svg,
                r##"<line x1="190" y1="{y:.3}" x2="200" y2="{y:.3}" stroke="#334155"/><text x="182" y="{:.3}" text-anchor="end" font-size="23" fill="#334155">{label}</text>"##,
                y + 8.
            )?;
            tick_rows.push(json!({"log10_coordinate":t,"original_value":raw,"label":label}));
        }
        let [x0, x1] = self.x_axis.bounds();
        let candidates: Vec<u64> = if x0 == x1 {
            vec![x0]
        } else {
            let mut v = vec![x0];
            let mut decade = 1_u64;
            while decade <= x1 + 1 {
                let original = match self.x_axis {
                    Abscissa::Generation { .. } => decade - 1,
                    Abscissa::FileOrdinal { .. } => decade,
                };
                if original > x0 && original < x1 {
                    v.push(original);
                }
                decade = match decade.checked_mul(10) {
                    Some(next) => next,
                    None => break,
                };
            }
            v.push(x1);
            v
        };
        for x in candidates {
            let px = self.position(x, self.original_domain[0])?[0];
            let label = match self.x_axis {
                Abscissa::Generation { .. } => format!("G{x}"),
                Abscissa::FileOrdinal { .. } => format!("{x}"),
            };
            write!(
                svg,
                r##"<text x="{}" y="607" text-anchor="middle" font-size="23" fill="#334155">{label}</text>"##,
                200 + px
            )?;
        }
        if matches!(self.x_axis, Abscissa::Generation { .. }) {
            let mut best: Option<f64> = None;
            let mut previous: Option<(u64, [usize; 2])> = None;
            for (&x, &value) in &self.best_by_x {
                let current = match best {
                    None => value,
                    Some(old) => {
                        if self.preference == Preference::Higher {
                            old.max(value)
                        } else {
                            old.min(value)
                        }
                    }
                };
                let pixel = self.position(x, current)?;
                if let Some((last, old)) = previous {
                    if last.checked_add(1) == Some(x) {
                        write!(
                            svg,
                            r##"<path d="M{},{} H{} V{}" fill="none" stroke="#0072b2" stroke-width="3"/>"##,
                            200 + old[0],
                            150 + old[1],
                            200 + pixel[0],
                            150 + pixel[1]
                        )?;
                    }
                }
                previous = Some((x, pixel));
                best = Some(current);
            }
        }
        let formula = self.axis.metadata()["coordinate_formula"]
            .as_str()
            .unwrap()
            .to_string();
        let x_label = match self.x_axis {
            Abscissa::Generation { .. } => {
                "Original generation; horizontal position = log10(generation + 1)"
            }
            Abscissa::FileOrdinal { .. } => {
                "Original one-based birth-file ordinal; horizontal position = log10(ordinal)"
            }
        };
        let direction = if self.preference == Preference::Higher {
            "Higher original score preferred"
        } else {
            "Lower original deficit preferred"
        };
        let movement = if self.gap {
            "Decreasing fixed-reference gap is better"
        } else if self.preference == Preference::Higher {
            "Increasing affine coordinate is better"
        } else {
            "Decreasing affine coordinate is better"
        };
        let overlay_label = match self.x_axis {
            Abscissa::Generation { .. } => {
                format!("Blue: cumulative best from these rows. {movement}.")
            }
            Abscissa::FileOrdinal { .. } => {
                "No trajectory: accession order is not an admitted campaign/time sequence.".into()
            }
        };
        let leader_label = match self.x_axis {
            Abscissa::Generation { .. } => format!(
                "{direction}; final leader first at G{}; {} exact tied rows.",
                self.first_leader_x.unwrap(),
                self.leader_ties
            ),
            Abscissa::FileOrdinal { .. } => format!(
                "{direction}; {} exact minimum ties. Ordinals identify source files.",
                self.leader_ties
            ),
        };
        write!(
            svg,
            r##"<text x="675" y="648" text-anchor="middle" font-size="23" fill="#334155">{x_label}</text><text x="30" y="115" font-size="23" fill="#334155">Raw score ticks; vertical position = log10({formula}); epsilon = 1e-6.</text><text x="30" y="690" font-size="23" fill="#334155">{overlay_label}</text><text x="30" y="730" font-size="23" fill="#334155">{leader_label}</text><text x="30" y="770" font-size="23" fill="#334155">Fixed full-source min/max and final reference; no physical ratios or cross-run ranking.</text><text x="30" y="810" font-size="23" fill="#334155">Unknown screen values are nonnumeric; full-range asinh and summary views are separate.</text></svg>\n"##
        )?;
        receipt["tick_rows"] = json!(tick_rows);
        receipt["svg_sha256"] = json!(format!("{:x}", Sha256::digest(svg.as_bytes())));
        Ok((svg, receipt))
    }
}
