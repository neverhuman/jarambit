//! Canonical crescent genome columns (`g.*`) and genome-derived features (`gd.*`).
//!
//! Pure std. The 128 gene names mirror `jarambit_crescent::genome` exactly; a test in
//! `datasets::featurize` fails if the two lists ever drift. Derived features are a pure
//! function of the genes, so they are recomputed at build time and can evolve without
//! touching committed records.

pub const N_CP: usize = 40;

pub const NAMES: [&str; 88] = [
    "blade_drop_deg", "grip_dx", "ring_wall", "ring_id", "neck_fillet", "t_stock", "t_tip", "taper_x0",
    "edge_angle_in", "edge_angle_sp", "comet_x0", "comet_x1", "comet_w0", "comet_w1", "comet_bow", "comet_y", "eye_scale", "cairn_d",
    "nub_engage", "holder_wall", "jimp_scale", "serr_large_d", "serr_small_d", "serr_fine_d", "spine_sharp_offset",
    "relief_d", "relief_w", "relief_q", "lattice_d", "lattice_pitch", "lattice_frac", "lattice_angle", "lattice_handle", "lattice_blade",
    "fuller_d", "fuller_w", "fuller_s0", "fuller_s1", "flute_d", "texture_mode", "texture_d", "texture_pitch", "texture_angle",
    "wrap_x0", "wrap_len", "pad_x", "pad_y", "pad_w", "pad_l", "edge_t",
    "iso_d", "iso_pitch", "iso_rib", "iso_angle", "iso_rim", "iso_blade", "iso_handle", "iso_blade_extent", "iso_grade",
    "win_n", "win_s0", "win_s1", "win_w", "win_fill", "win_decay", "rflute_d", "rflute_w", "rflute_arc", "iso_handle_start", "iso_land", "perf_d", "perf_gap", "perf_angle", "perf_elong", "perf_land", "perf_handle", "perf_handle_start",
    "p1_on", "rail_w", "neck_l", "pocket_d", "pocket_r", "k_n", "k0", "k1", "k2", "k3", "k4",
];

/// Material flag column (0 = Ti-6Al-4V, 1 = Ti-3Al-2.5V).
pub const MATERIAL_COL: &str = "g.mat_ti3al25v";

/// All 129 canonical genome columns, in genome order, with the `g.` prefix.
pub fn gene_columns() -> Vec<String> {
    let mut v: Vec<String> = (0..N_CP).map(|i| format!("g.cp{i:02}")).collect();
    v.extend(NAMES.iter().map(|n| format!("g.{n}")));
    v.push(MATERIAL_COL.to_string());
    v
}

/// Map a raw genome CSV header name (`cp07`, `ring_wall`, `material`) to its record column.
pub fn record_column_for_gene_header(name: &str) -> Option<String> {
    let n = name.trim();
    if n == "material" {
        return Some(MATERIAL_COL.to_string());
    }
    if let Some(i) = n.strip_prefix("cp").and_then(|d| d.parse::<usize>().ok()) {
        return (i < N_CP).then(|| format!("g.cp{i:02}"));
    }
    NAMES.contains(&n).then(|| format!("g.{n}"))
}

pub fn material_flag(name: &str) -> f64 {
    if name.contains("3Al") {
        1.0
    } else if name.trim().starts_with("Ti") {
        0.0
    } else {
        f64::NAN
    }
}

/// Stable 64-bit content hash of a complete genome (129 values in canonical order), hex encoded.
/// Returns None when any gene is missing.
pub fn genome_hash(genes: &[f64]) -> Option<String> {
    if genes.len() != N_CP + NAMES.len() + 1 || genes.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let mut bytes = Vec::with_capacity(genes.len() * 8 + 8);
    bytes.extend_from_slice(b"jarambit.genome.v1");
    for x in genes {
        // +0.0 and -0.0 are the same gene value
        let x = if *x == 0.0 { 0.0f64 } else { *x };
        bytes.extend_from_slice(&x.to_bits().to_le_bytes());
    }
    Some(format!("{:016x}", super::stats::fnv1a(&bytes)))
}

/// Envelope half-width of each silhouette control point (mirror of `genome::cp_delta`).
pub fn cp_delta(i: usize) -> f64 {
    match i {
        0 | 39 | 19..=22 => 0.0,
        1..=7 => 3.0,
        8 => 2.0,
        9..=13 => 3.5,
        14..=18 => 3.0,
        23..=27 => 3.0,
        28 | 29 => 2.0,
        30..=35 => 3.5,
        36..=38 => 2.5,
        _ => 0.0,
    }
}

const ZONES: [(&str, usize, usize); 6] =
    [("tip_inside", 1, 7), ("belly", 8, 13), ("heel_in", 14, 18), ("spine_rear", 23, 29), ("spine_mid", 30, 35), ("spine_tip", 36, 38)];

/// Genome-derived features from the 129 canonical genes (same order as `gene_columns`).
pub fn derived(genes: &[f64]) -> Vec<(String, f64)> {
    let mut out: Vec<(String, f64)> = Vec::new();
    if genes.len() != N_CP + NAMES.len() + 1 {
        return out;
    }
    let cp = &genes[..N_CP];
    let named = |n: &str| -> f64 {
        NAMES.iter().position(|x| *x == n).map(|i| genes[N_CP + i]).unwrap_or(f64::NAN)
    };
    let mut push = |k: &str, v: f64| out.push((format!("gd.{k}"), v));

    // silhouette: normalized offsets per zone
    let u: Vec<f64> = (0..N_CP).map(|i| if cp_delta(i) > 0.0 { cp[i] / cp_delta(i) } else { 0.0 }).collect();
    for (name, a, b) in ZONES {
        let s = &u[a..=b];
        push(&format!("sil_{name}_mean"), s.iter().sum::<f64>() / s.len() as f64);
        push(&format!("sil_{name}_absmean"), s.iter().map(|x| x.abs()).sum::<f64>() / s.len() as f64);
    }
    push("sil_energy", u.iter().map(|x| x * x).sum());
    let n = N_CP;
    let rough: f64 = (0..n).map(|i| (cp[(i + 1) % n] - 2.0 * cp[i] + cp[(i + n - 1) % n]).powi(2)).sum();
    push("sil_roughness", rough);
    push("sil_total_variation", (0..n).map(|i| (cp[(i + 1) % n] - cp[i]).abs()).sum());
    let (imax, vmax) = cp.iter().enumerate().fold((0, f64::MIN), |a, (i, &v)| if v > a.1 { (i, v) } else { a });
    let (imin, vmin) = cp.iter().enumerate().fold((0, f64::MAX), |a, (i, &v)| if v < a.1 { (i, v) } else { a });
    push("sil_max", vmax);
    push("sil_min", vmin);
    push("sil_argmax_frac", imax as f64 / n as f64);
    push("sil_argmin_frac", imin as f64 / n as f64);
    // low-order Fourier description of the closed offset field
    for k in 1..=6usize {
        let (mut a, mut b) = (0.0, 0.0);
        for (i, v) in cp.iter().enumerate() {
            let th = std::f64::consts::TAU * (k * i) as f64 / n as f64;
            a += v * th.cos();
            b += v * th.sin();
        }
        a *= 2.0 / n as f64;
        b *= 2.0 / n as f64;
        if k <= 4 {
            push(&format!("sil_fourier{k}_cos"), a);
            push(&format!("sil_fourier{k}_sin"), b);
        }
        push(&format!("sil_fourier{k}_amp"), a.hypot(b));
    }

    // section / stock
    let t = named("t_stock");
    let ring_id = named("ring_id");
    let ring_wall = named("ring_wall");
    push("t_minus_tip", t - named("t_tip"));
    push("ring_od", ring_id + 2.0 * ring_wall);
    push("ring_annulus_mm2", std::f64::consts::PI / 4.0 * ((ring_id + 2.0 * ring_wall).powi(2) - ring_id.powi(2)));
    push("ring_wall_x_t", ring_wall * t);
    push("edge_angle_mean", 0.5 * (named("edge_angle_in") + named("edge_angle_sp")));
    push("edge_angle_diff", named("edge_angle_in") - named("edge_angle_sp"));
    push("comet_span", named("comet_x1") - named("comet_x0"));
    push("comet_taper", named("comet_w0") - named("comet_w1"));
    push("fuller_span", named("fuller_s1") - named("fuller_s0"));
    push("win_span", named("win_s1") - named("win_s0"));

    // architecture switches as the phenotype interprets them
    let on = |x: f64| if x >= 0.5 { 1.0 } else { 0.0 };
    let pos = |x: f64| if x > 0.0 { 1.0 } else { 0.0 };
    let p1 = on(named("p1_on"));
    let iso = pos(named("iso_d")) * on(named("iso_blade").max(named("iso_handle")).max(named("iso_land")));
    let win = named("win_n").floor().max(0.0);
    let perf = pos(named("perf_d")) * on(named("perf_land").max(named("perf_handle")));
    let lat = pos(named("lattice_d")) * on(named("lattice_handle").max(named("lattice_blade")));
    let relief = pos(named("relief_d"));
    let tex = on(named("texture_mode")) * pos(named("texture_d"));
    let fuller = pos(named("fuller_d"));
    let rflute = pos(named("rflute_d"));
    let flute = pos(named("flute_d"));
    push("p1", p1);
    push("iso_on", iso);
    push("win_count", win);
    push("perf_on", perf);
    push("lattice_on", lat);
    push("relief_on", relief);
    push("texture_on", tex);
    push("fuller_on", fuller);
    push("rflute_on", rflute);
    push("flute_on", flute);
    push("machining_features_on", iso + pos(win) + perf + lat + relief + tex + fuller + rflute + flute);
    push("iso_depth_density", named("iso_d") / named("iso_pitch").max(1e-9) * iso);
    push("perf_density", named("perf_d") / (named("perf_d") + named("perf_gap")).max(1e-9) * perf);

    // P1 thickness knots
    let kn = named("k_n").floor().clamp(3.0, 5.0) as usize;
    let knots: Vec<f64> = (0..kn).map(|k| named(&format!("k{k}"))).collect();
    let kmin = knots.iter().cloned().fold(f64::INFINITY, f64::min);
    let kmax = knots.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    push("k_count", kn as f64);
    push("knot_min", kmin);
    push("knot_max", kmax);
    push("knot_mean", knots.iter().sum::<f64>() / kn as f64);
    push("knot_range", kmax - kmin);
    push("knot_min_minus_t", kmin - t);
    out
}

/// Names produced by `derived` (stable order); used to declare table columns before any row exists.
pub fn derived_names() -> Vec<String> {
    let mut genes = vec![0.0; N_CP + NAMES.len() + 1];
    genes[N_CP + NAMES.iter().position(|n| *n == "k_n").expect("k_n")] = 5.0;
    derived(&genes).into_iter().map(|(k, _)| k).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gene_columns_are_complete_and_mappable() {
        let cols = gene_columns();
        assert_eq!(cols.len(), 129);
        assert_eq!(cols[0], "g.cp00");
        assert_eq!(cols[40], "g.blade_drop_deg");
        assert_eq!(cols[127], "g.k4");
        assert_eq!(record_column_for_gene_header("cp07").as_deref(), Some("g.cp07"));
        assert_eq!(record_column_for_gene_header("material").as_deref(), Some(MATERIAL_COL));
        assert_eq!(record_column_for_gene_header("not_a_gene"), None);
        assert_eq!(material_flag("Ti-3Al-2.5V"), 1.0);
        assert_eq!(material_flag("Ti-6Al-4V"), 0.0);
    }

    #[test]
    fn genome_hash_is_stable_and_sign_of_zero_insensitive() {
        let mut g = vec![1.25; 129];
        let h = genome_hash(&g).unwrap();
        assert_eq!(h.len(), 16);
        g[3] = -0.0;
        let a = genome_hash(&g).unwrap();
        g[3] = 0.0;
        assert_eq!(a, genome_hash(&g).unwrap());
        g[5] = f64::NAN;
        assert!(genome_hash(&g).is_none());
        assert!(genome_hash(&[1.0; 10]).is_none());
    }

    #[test]
    fn derived_features_are_finite_and_named_consistently() {
        let mut g = vec![0.0; 129];
        for (i, v) in g.iter_mut().enumerate().take(N_CP) {
            *v = cp_delta(i) * (i as f64 * 0.3).sin();
        }
        let set = |g: &mut Vec<f64>, n: &str, v: f64| g[N_CP + NAMES.iter().position(|x| *x == n).unwrap()] = v;
        set(&mut g, "t_stock", 5.7);
        set(&mut g, "t_tip", 3.0);
        set(&mut g, "ring_id", 25.4);
        set(&mut g, "ring_wall", 6.3);
        set(&mut g, "k_n", 4.2);
        for k in 0..5 {
            set(&mut g, &format!("k{k}"), 5.7 - 0.1 * k as f64);
        }
        set(&mut g, "iso_pitch", 10.0);
        let d = derived(&g);
        assert_eq!(d.iter().map(|(k, _)| k.clone()).collect::<Vec<_>>(), derived_names());
        assert!(d.iter().all(|(_, v)| v.is_finite()), "{d:?}");
        let get = |k: &str| d.iter().find(|(n, _)| n == k).unwrap().1;
        assert!((get("gd.ring_od") - 38.0).abs() < 1e-12);
        assert_eq!(get("gd.k_count"), 4.0);
        assert!((get("gd.knot_range") - 0.3).abs() < 1e-12);
        assert!(get("gd.sil_fourier1_amp") > 0.0);
    }
}
