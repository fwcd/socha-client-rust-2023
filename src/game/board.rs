use crate::util::{Element, SCError, SCResult};

use super::Field;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // TODO: Make this a flat array for efficiency and expose 2D indexing methods
    fields: Vec<Vec<Field>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::empty()
    }
}

impl Board {
    /// Creates an empty board.
    pub fn empty() -> Self {
        Self { fields: Vec::new() }
    }

    /// Creates a new board with the given fields.
    pub fn new(fields: Vec<Vec<Field>>) -> Self {
        Self { fields }
    }
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Self {
            fields: elem.childs_by_name("list")
                .map(|c| c.childs_by_name("field").map(|c| c.try_into()).collect())
                .collect::<SCResult<_>>()?
        })
    }
}
