use std::ops::Range;
use derive_more::From;


#[derive(Debug, PartialEq, From)]
pub enum MarinValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(Vec<MarinValue>),
    Range(Range<i64>),
}

impl From<&str> for MarinValue {
    fn from(s: &str) -> Self {
        MarinValue::String(s.into())
    }
}
