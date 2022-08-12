use std::str::FromStr;

use crate::util::{SCError, SCResult};

/// Determines the cause of a game score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreCause {
    Regular,
    Left,
    RuleViolation,
    SoftTimeout,
    HardTimeout,
    Unknown
}

impl FromStr for ScoreCause {
    type Err = SCError;

    fn from_str(raw: &str) -> SCResult<Self> {
        match raw {
            "REGULAR" => Ok(Self::Regular),
            "LEFT" => Ok(Self::Left),
            "RULE_VIOLATION" => Ok(Self::RuleViolation),
            "SOFT_TIMEOUT" => Ok(Self::SoftTimeout),
            "HARD_TIMEOUT" => Ok(Self::HardTimeout),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Err(SCError::UnknownVariant(format!("Unknown score cause {}", raw)))
        }
    }
}
