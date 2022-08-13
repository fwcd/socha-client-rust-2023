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
        "#).unwrap()).unwrap(), Board::new(vec![
            vec![3.into(), 2.into(), 1.into(), 1.into(), 4.into(), 3.into(), 2.into(), 3.into()],
            vec![3.into(), 2.into(), 2.into(), 3.into(), 1.into(), 1.into(), 2.into(), 1.into()],
            vec![1.into(), 2.into(), 2.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into()],
            vec![2.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into()],
            vec![1.into(), 1.into(), 1.into(), 1.into(), 2.into(), 1.into(), 1.into(), 1.into()],
            vec![1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), Team::One.into(), 1.into()],
            vec![1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into()],
            vec![1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into(), 1.into()],
        ]));
    }
}
