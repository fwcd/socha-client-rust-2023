use std::str::FromStr;
use std::fmt;

use crate::util::{SCError, SCResult};

/// A playing party in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    One,
    Two,
}

impl Team {
    /// The team's index.
    pub fn index(self) -> i32 {
        match self {
            Team::One => 0,
            Team::Two => 1,
        }
    }

    /// The opponent of the given team.
    pub fn opponent(self) -> Team {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }

    /// The x-direction of the team on the board.
    pub fn direction(self) -> i32 {
        match self {
            Team::One => 1,
            Team::Two => -1,
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::One => write!(f, "ONE"),
            Team::Two => write!(f, "TWO"),
        }
    }
}

impl FromStr for Team {
    type Err = SCError;

    fn from_str(s: &str) -> SCResult<Self> {
        match s {
            "ONE" => Ok(Team::One),
            "TWO" => Ok(Team::Two),
            _ => Err(SCError::UnknownVariant(format!("Unknown team {}", s))),
        }
    }
}
