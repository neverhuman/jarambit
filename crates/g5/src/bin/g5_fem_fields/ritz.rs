//! Buckling of the point: the Rayleigh-Ritz column of `loads::tip_buckling_load_n` (`crates/g5/src/loads.rs` at
//! 48d3a8c7, hashed into the G5 decoder), copied with eigenvector accumulation added so the mode can be plotted.
//! The eigenvalue is computed by the original operations (the rotations of A are unchanged; the eigenvector matrix
//! is rotated alongside), and it is checked against the record bit for bit.

pub fn tip_buckling_load_n(ei: &dyn Fn(f64) -> f64, s0: f64, len: f64) -> (f64, [f64; 7]) {
    const NB: usize = 7;
    let nq = 400usize;
    let l = len - s0;
    let mut k = [[0.0_f64; NB]; NB];
    let mut g = [[0.0_f64; NB]; NB];
    for q in 0..nq {
        let xi = (q as f64 + 0.5) / nq as f64;
        let s = s0 + xi * l;
        let w = l / nq as f64;
        let e = ei(s);
        let mut d1 = [0.0; NB];
        let mut d2 = [0.0; NB];
        for (i, (a, c)) in d1.iter_mut().zip(d2.iter_mut()).enumerate() {
            let pw = (i + 2) as f64;
            *a = -pw * (1.0 - xi).powf(pw - 1.0) / l;
            *c = pw * (pw - 1.0) * (1.0 - xi).powf(pw - 2.0) / (l * l);
        }
        for i in 0..NB {
            for jj in 0..NB {
                k[i][jj] += e * d2[i] * d2[jj] * w;
                g[i][jj] += d1[i] * d1[jj] * w;
            }
        }
    }
    smallest_generalized_eigenvalue(&k, &g)
}

/// The buckling mode w(s) of the Ritz coefficients `a`, at `n` points from `s0` (the loaded point end) to `len`.
pub fn mode_shape(a: &[f64; 7], s0: f64, len: f64, n: usize) -> Vec<(f64, f64)> {
    let l = len - s0;
    (0..=n)
        .map(|q| {
            let xi = q as f64 / n as f64;
            let w: f64 = (0..7).map(|i| a[i] * (1.0 - xi).powf((i + 2) as f64)).sum();
            (s0 + xi * l, w)
        })
        .collect()
}

/// Smallest λ of K a = λ G a for symmetric K and SPD G (Cholesky reduction, then Jacobi).
fn smallest_generalized_eigenvalue<const N: usize>(k: &[[f64; N]; N], g: &[[f64; N]; N]) -> (f64, [f64; N]) {
    let mut lch = [[0.0_f64; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let mut sum = g[i][j];
            for kk in 0..j {
                sum -= lch[i][kk] * lch[j][kk];
            }
            if i == j {
                if sum <= 0.0 {
                    return (f64::NAN, [f64::NAN; N]);
                }
                lch[i][i] = sum.sqrt();
            } else {
                lch[i][j] = sum / lch[j][j];
            }
        }
    }
    // C = L^-1 K L^-T
    let mut y = [[0.0_f64; N]; N]; // Y = L^-1 K
    for c in 0..N {
        for i in 0..N {
            let mut sum = k[i][c];
            for kk in 0..i {
                sum -= lch[i][kk] * y[kk][c];
            }
            y[i][c] = sum / lch[i][i];
        }
    }
    let mut a = [[0.0_f64; N]; N]; // A = Y L^-T  (solve A L^T = Y row by row)
    for r in 0..N {
        for i in 0..N {
            let mut sum = y[r][i];
            for kk in 0..i {
                sum -= a[r][kk] * lch[i][kk];
            }
            a[r][i] = sum / lch[i][i];
        }
    }
    // eigenvectors of A: the product of the Jacobi rotations (added; A itself is rotated exactly as before)
    let mut v = [[0.0_f64; N]; N];
    for (i, row) in v.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for _sweep in 0..100 {
        let mut off = 0.0;
        for i in 0..N {
            for j in (i + 1)..N {
                off += a[i][j] * a[i][j];
            }
        }
        if off < 1e-24 {
            break;
        }
        for pp in 0..N {
            for qq in (pp + 1)..N {
                if a[pp][qq].abs() < 1e-300 {
                    continue;
                }
                let theta = (a[qq][qq] - a[pp][pp]) / (2.0 * a[pp][qq]);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let t = if theta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for r in 0..N {
                    let (arp, arq) = (a[r][pp], a[r][qq]);
                    a[r][pp] = c * arp - s * arq;
                    a[r][qq] = s * arp + c * arq;
                }
                for r in 0..N {
                    let (apr, aqr) = (a[pp][r], a[qq][r]);
                    a[pp][r] = c * apr - s * aqr;
                    a[qq][r] = s * apr + c * aqr;
                }
                for r in 0..N {
                    let (vrp, vrq) = (v[r][pp], v[r][qq]);
                    v[r][pp] = c * vrp - s * vrq;
                    v[r][qq] = s * vrp + c * vrq;
                }
            }
        }
    }
    let lambda = (0..N).map(|i| a[i][i]).fold(f64::INFINITY, f64::min);
    // the column of the smallest diagonal entry, mapped back: K a = lambda G a with a = L^-T y
    let m = (0..N).find(|&i| a[i][i] == lambda).unwrap_or(0);
    let mut x = [0.0_f64; N];
    for i in (0..N).rev() {
        let mut sum = v[i][m];
        for kk in (i + 1)..N {
            sum -= lch[kk][i] * x[kk];
        }
        x[i] = sum / lch[i][i];
    }
    (lambda, x)
}
