use crate::util::{Element, SCError, SCResult};

use super::{Field, BOARD_FIELDS};

// Ported from https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Board.kt

/// The 8x8 game board, a two-dimensional grid of ice floes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    fields: [Field; BOARD_FIELDS],
}

impl Default for Board {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Board {
    /// The empty board.
    pub const EMPTY: Self = Self { fields: [Field::EMPTY; BOARD_FIELDS] };

    /// Creates a new board with the given fields.
    pub const fn new(fields: [Field; BOARD_FIELDS]) -> Self {
        Self { fields }
    }
}

impl TryFrom<&Element> for Board {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Self {
            fields: elem.childs_by_name("list")
                .flat_map(|c| c.childs_by_name("field").map(|c| c.try_into()))
                .collect::<SCResult<Vec<Field>>>()?
                .try_into()
                .map_err(|e| SCError::from(format!("Board has wrong number of fields: {:?}", e)))?
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Board, Team}};

    #[test]
    fn test_parsing_board() {
        assert_eq!(Board::try_from(&Element::from_str(r#"
            <board>
                <list>
                    <field>3</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>4</field>
                    <field>3</field>
                    <field>2</field>
                    <field>3</field>
                </list>
                <list>
                    <field>3</field>
                    <field>2</field>
                    <field>2</field>
                    <field>3</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>2</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>2</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>ONE</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
                <list>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                    <field>1</field>
                </list>
            </board>
        "#).unwrap()).unwrap(), Board::new([
            3.into(), 2.into(), 1.into(), 1.into(), 4.into(), 3.into(), 2.into(), 3.into(),
            3.into(), 2.into(), 2.into(), 3.into(), 1.into(), 1.into(), 2.into(), 1.into(),
            1.into(), 2.into(), 2.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into(),
            2.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), Team::One.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
            1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(),
        ]));
    }
}
