//! Out-of-fold regression statistics used to decide whether a dataset shows lift.

/// Pearson correlation; NaN when either side is constant or n < 3.
pub fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 3 {
        return f64::NAN;
    }
    let mx = x[..n].iter().sum::<f64>() / n as f64;
    let my = y[..n].iter().sum::<f64>() / n as f64;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (dx, dy) = (x[i] - mx, y[i] - my);
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return f64::NAN;
    }
    (sxy / (sxx * syy).sqrt()).clamp(-1.0, 1.0)
}

/// Average ranks (ties share the mean rank), 1-based.
pub fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
    let mut out = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            out[idx[k]] = r;
        }
        i = j + 1;
    }
    out
}

pub fn spearman(x: &[f64], y: &[f64]) -> f64 {
    pearson(&ranks(x), &ranks(y))
}

pub fn rmse(y: &[f64], p: &[f64]) -> f64 {
    if y.is_empty() {
        return f64::NAN;
    }
    (y.iter().zip(p).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / y.len() as f64).sqrt()
}

pub fn mae(y: &[f64], p: &[f64]) -> f64 {
    if y.is_empty() {
        return f64::NAN;
    }
    y.iter().zip(p).map(|(a, b)| (a - b).abs()).sum::<f64>() / y.len() as f64
}

/// Coefficient of determination against the global mean of `y` (can be negative).
pub fn r2(y: &[f64], p: &[f64]) -> f64 {
    if y.len() < 2 {
        return f64::NAN;
    }
    let m = y.iter().sum::<f64>() / y.len() as f64;
    let sst: f64 = y.iter().map(|a| (a - m).powi(2)).sum();
    if sst <= 0.0 {
        return f64::NAN;
    }
    1.0 - y.iter().zip(p).map(|(a, b)| (a - b).powi(2)).sum::<f64>() / sst
}

/// Two-sided 95 % confidence interval of a Pearson r via the Fisher z transform.
pub fn fisher_ci95(r: f64, n: usize) -> (f64, f64) {
    if !r.is_finite() || n < 4 {
        return (f64::NAN, f64::NAN);
    }
    let z = r.clamp(-0.999_999_9, 0.999_999_9).atanh();
    let se = 1.0 / ((n as f64) - 3.0).sqrt();
    ((z - 1.959_963_984_540_054 * se).tanh(), (z + 1.959_963_984_540_054 * se).tanh())
}

/// Lift verdict for one dataset.
/// `lift`: OOF r > 0 with the 95 % CI excluding zero AND OOF predictions beat the fold-mean baseline.
/// `weak`: positive r but the CI touches zero or the baseline is not beaten.
/// `none`: otherwise (including undefined r).
pub fn verdict(r: f64, ci_lo: f64, rmse_model: f64, rmse_baseline: f64) -> &'static str {
    if !r.is_finite() || r <= 0.0 {
        return "none";
    }
    if ci_lo > 0.0 && rmse_model < rmse_baseline {
        "lift"
    } else {
        "weak"
    }
}

/// Practical strength of a `lift` verdict from the out-of-fold R²: `strong` (R² >= 0.5),
/// `moderate` (>= 0.2) or `marginal`; anything that is not `lift` has no strength.
pub fn strength(verdict: &str, r2: f64) -> &'static str {
    if verdict != "lift" {
        "-"
    } else if r2 >= 0.5 {
        "strong"
    } else if r2 >= 0.2 {
        "moderate"
    } else {
        "marginal"
    }
}

/// Deterministic SplitMix64 stream (same constants as `jarambit_genotype::Rng`).
pub struct SplitMix(pub u64);
impl SplitMix {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Fisher-Yates permutation of 0..n.
    pub fn permutation(&mut self, n: usize) -> Vec<usize> {
        let mut v: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
        v
    }
}

/// FNV-1a 64-bit over bytes.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pearson_and_spearman_match_hand_values() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert!((pearson(&x, &y) - 1.0).abs() < 1e-12);
        let z = [5.0, 4.0, 3.0, 2.0, 1.0];
        assert!((pearson(&x, &z) + 1.0).abs() < 1e-12);
        assert!(pearson(&x, &[1.0; 5]).is_nan());
        assert_eq!(ranks(&[10.0, 20.0, 20.0, 5.0]), vec![2.0, 3.5, 3.5, 1.0]);
        let y2 = [1.0, 4.0, 9.0, 16.0, 25.0];
        assert!((spearman(&x, &y2) - 1.0).abs() < 1e-12);
        // r = 0.8 for this classic table (x, y)
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let b = [2.0, 1.0, 4.0, 3.0, 5.0];
        assert!((pearson(&a, &b) - 0.8).abs() < 1e-12);
    }

    #[test]
    fn error_metrics_and_r2() {
        let y = [1.0, 2.0, 3.0];
        assert_eq!(rmse(&y, &y), 0.0);
        assert!((r2(&y, &y) - 1.0).abs() < 1e-12);
        assert!((r2(&y, &[2.0, 2.0, 2.0])).abs() < 1e-12);
        assert!((mae(&y, &[2.0, 2.0, 2.0]) - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn fisher_interval_and_verdicts() {
        let (lo, hi) = fisher_ci95(0.5, 103);
        assert!(lo > 0.33 && lo < 0.36 && hi > 0.62 && hi < 0.64, "{lo} {hi}");
        let (lo_small, _) = fisher_ci95(0.3, 10);
        assert!(lo_small < 0.0);
        assert_eq!(verdict(0.5, lo, 1.0, 2.0), "lift");
        assert_eq!(verdict(0.3, lo_small, 1.0, 2.0), "weak");
        assert_eq!(verdict(0.5, lo, 2.5, 2.0), "weak");
        assert_eq!(verdict(-0.1, -0.3, 1.0, 2.0), "none");
        assert_eq!(verdict(f64::NAN, f64::NAN, 1.0, 2.0), "none");
        assert_eq!(strength("lift", 0.81), "strong");
        assert_eq!(strength("lift", 0.3), "moderate");
        assert_eq!(strength("lift", -0.02), "marginal");
        assert_eq!(strength("weak", 0.9), "-");
    }

    #[test]
    fn splitmix_permutation_is_deterministic_and_complete() {
        let a = SplitMix(42).permutation(100);
        let b = SplitMix(42).permutation(100);
        assert_eq!(a, b);
        let mut s = a.clone();
        s.sort();
        assert_eq!(s, (0..100).collect::<Vec<_>>());
        assert_ne!(a, SplitMix(43).permutation(100));
        assert_eq!(fnv1a(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
    }
}
