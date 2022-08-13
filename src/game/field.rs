use crate::util::{Element, SCError, SCResult};

use super::Team;

// Port of https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Field.kt

/// A field on the board.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Field {
    fish: usize,
    penguin: Option<Team>,
}

impl Default for Field {
    fn default() -> Self {
        Self { fish: 0, penguin: None }
    }
}

impl Field {
    /// Creates a new field with the given fish.
    pub fn with_fish(fish: usize) -> Self {
        Self { fish, ..Default::default() }
    }

    /// Creates a new field with the given penguin.
    pub fn with_penguin(team: Team) -> Self {
        Self { penguin: Some(team), ..Default::default() }
    }

    /// Whether the field is empty.
    pub fn is_empty(self) -> bool { self.fish == 0 && self.penguin.is_none() }

    /// Whether the field is occupied by a penguin.
    pub fn is_occupied(self) -> bool { self.penguin.is_some() }

    /// The number of fish on this field.
    pub fn fish(self) -> usize { self.fish }

    /// The penguin on this field.
    pub fn penguin(self) -> Option<Team> { self.penguin }
}

impl From<usize> for Field {
    fn from(fish: usize) -> Self {
        Self::with_fish(fish)
    }
}

impl From<Team> for Field {
    fn from(team: Team) -> Self {
        Self::with_penguin(team)
    }
}

impl TryFrom<&Element> for Field {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Self {
            fish: elem.content().parse().unwrap_or(0),
            penguin: elem.content().parse().ok(),
        })
    }
}
