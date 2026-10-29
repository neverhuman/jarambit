//! Exact JSON for f64 values that may be non-finite: numbers, or "inf" / "-inf" / "nan" (copied from G5.0's
//! `loads::f64x`, which is private), so a row with an infinite safety factor re-reads bit for bit.
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.is_finite() {
        s.serialize_f64(*v)
    } else if v.is_nan() {
        s.serialize_str("nan")
    } else if *v > 0.0 {
        s.serialize_str("inf")
    } else {
        s.serialize_str("-inf")
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Repr {
    N(f64),
    S(String),
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    match Repr::deserialize(d)? {
        Repr::N(v) => Ok(v),
        Repr::S(t) => match t.as_str() {
            "inf" => Ok(f64::INFINITY),
            "-inf" => Ok(f64::NEG_INFINITY),
            "nan" => Ok(f64::NAN),
            other => Err(serde::de::Error::custom(format!("not an f64: {other}"))),
        },
    }
}
