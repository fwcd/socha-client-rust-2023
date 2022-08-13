use crate::util::{Element, SCError, SCResult};

use super::Team;

// Port of https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Field.kt

/// A field on the board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub fish: usize,
    pub penguin: Option<Team>,
}

impl Field {
    /// Whether the field is empty.
    pub fn is_empty(&self) -> bool {
        self.fish == 0 && self.penguin.is_none()
    }

    /// Whether the field is occupied by a penguin.
    pub fn is_occupied(&self) -> bool {
        self.penguin.is_some()
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
