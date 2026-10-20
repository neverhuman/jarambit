//! Display-axis rules for retained numeric observations, independent of selection.
use serde_json::{json, Value};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ScoreAxis {
    Linear,
    Asinh { linear_threshold: f64 },
}

impl ScoreAxis {
    pub fn derive(values: &[f64]) -> Result<Self> {
        let mut sorted = values.to_vec();
        if sorted.is_empty() || sorted.iter().any(|v| !v.is_finite()) {
            return Err("score axis: empty or nonfinite source".into());
        }
        sorted.sort_by(f64::total_cmp);
        let (low, high) = (sorted[0], sorted[sorted.len() - 1]);
        let one_sign = (low > 0. && high > 0.) || (low < 0. && high < 0.);
        if low == high
            || (one_sign && low.abs().max(high.abs()) / low.abs().min(high.abs()) <= 4.)
        {
            return Ok(Self::Linear);
        }
        let mut nonzero: Vec<_> = sorted.iter().map(|v| v.abs()).filter(|v| *v > 0.).collect();
        nonzero.sort_by(f64::total_cmp);
        let threshold = quantile_sorted(&nonzero, 0.25).unwrap_or(1.);
        if !threshold.is_finite() || threshold <= 0. {
            return Err("score axis: invalid robust linear threshold".into());
        }
        Ok(Self::Asinh {
            linear_threshold: threshold,
        })
    }

    pub fn transform(self, value: f64) -> f64 {
        match self {
            Self::Linear => value,
            Self::Asinh { linear_threshold } => {
                let ratio = value / linear_threshold;
                if ratio.is_finite() {
                    ratio.asinh()
                } else {
                    value.signum() * (value.abs().ln() - linear_threshold.ln() + 2_f64.ln())
                }
            }
        }
    }

    pub fn inverse(self, value: f64) -> f64 {
        match self {
            Self::Linear => value,
            Self::Asinh { linear_threshold } => {
                if value.abs() < 20. {
                    value.sinh() * linear_threshold
                } else {
                    value.signum() * (value.abs() + linear_threshold.ln() - 2_f64.ln()).exp()
                }
            }
        }
    }

    pub fn ticks(self, domain: [f64; 2]) -> Vec<f64> {
        if domain[0] == domain[1] {
            return vec![domain[0]];
        }
        (0..5)
            .map(|i| {
                self.inverse(
                    self.transform(domain[0])
                        + i as f64 / 4. * (self.transform(domain[1]) - self.transform(domain[0])),
                )
            })
            .collect()
    }

    pub fn metadata(self) -> Value {
        match self {
            Self::Linear => {
                json!({"transform":"linear","inverse":"identity","rule":"constant, or one-sign endpoint absolute ratio<=4"})
            }
            Self::Asinh { linear_threshold } => {
                json!({"transform":"asinh(y/L)","inverse":"L*sinh(t)","linear_threshold_L":linear_threshold,"L_rule":"empirical Q25 of nonzero absolute original observations; floor(p*(n-1)); zero excluded only from L estimation, never from points"})
            }
        }
    }
}

pub(crate) fn quantile_sorted(sorted: &[f64], probability: f64) -> Option<f64> {
    if sorted.is_empty() {
        None
    } else {
        Some(sorted[(probability * (sorted.len() - 1) as f64).floor() as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_axis_preserves_order_zero_and_inverse() {
        let axis = ScoreAxis::derive(&[-100., -2., 0., 1., 3.]).unwrap();
        assert_eq!(axis.transform(0.), 0.);
        for value in [-100., -1., 0., 1., 100.] {
            assert!((axis.inverse(axis.transform(value)) - value).abs() < 1e-10);
        }
        let points = [-100., -1., 0., 1., 100.].map(|v| axis.transform(v));
        assert!(points.windows(2).all(|v| v[0] < v[1]));
        assert!(ScoreAxis::derive(&[f64::INFINITY]).is_err());
        let extreme = ScoreAxis::Asinh {
            linear_threshold: 1e-300,
        };
        assert!(extreme.transform(1e300).is_finite());
        assert!((extreme.inverse(extreme.transform(1e300)) / 1e300 - 1.).abs() < 1e-12);
    }

    #[test]
    fn axes_keep_constants_and_one_sign_domains_linear() {
        assert!(matches!(ScoreAxis::derive(&[3.]).unwrap(), ScoreAxis::Linear));
        assert!(matches!(ScoreAxis::derive(&[2., 2., 2.]).unwrap(), ScoreAxis::Linear));
        assert_eq!(quantile_sorted(&[1., 2., 2., 9., 20.], 0.25), Some(2.));
        assert_eq!(quantile_sorted(&[], 0.25), None);
        assert!(matches!(
            ScoreAxis::derive(&[2., 3., 4.]).unwrap(),
            ScoreAxis::Linear
        ));
        for values in [[1e-300, 2e-300], [-2e-300, -1e-300]] {
            assert!(matches!(
                ScoreAxis::derive(&values).unwrap(),
                ScoreAxis::Linear
            ));
        }
        assert!(matches!(
            ScoreAxis::derive(&[-1e-300, 1e-300]).unwrap(),
            ScoreAxis::Asinh { .. }
        ));
    }
}
