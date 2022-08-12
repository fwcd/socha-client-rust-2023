use std::{fmt, str::FromStr};

use crate::util::{SCError, SCResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScoreAggregation {
    Sum,
    Average,
}

impl fmt::Display for ScoreAggregation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => write!(f, "SUM"),
            Self::Average => write!(f, "AVERAGE"),
        }
    }
}

impl FromStr for ScoreAggregation {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "SUM" => Ok(Self::Sum),
            "AVERAGE" => Ok(Self::Average),
            _ => Err(SCError::UnknownVariant(format!("Unknown aggregation {}", s))),
        }
    }
}
