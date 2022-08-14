use std::ops::{Index, IndexMut};

use crate::util::{Element, SCError, SCResult};

use super::{Field, BOARD_FIELDS, Vec2, Direct, BOARD_SIZE, Move, Doubled, Team};

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

    /// Checks whether the given coordinates are in bounds.
    pub fn in_bounds(coords: impl Into<Vec2<Direct>>) -> bool {
        let direct: Vec2<Direct> = coords.into();
        direct.x >= 0 && direct.x < BOARD_SIZE as i32 && direct.y >= 0 && direct.y < BOARD_SIZE as i32
    }

    /// Converts coordinates to an index.
    fn index_for(coords: impl Into<Vec2<Direct>>) -> usize {
        let direct: Vec2<Direct> = coords.into();
        direct.y as usize * BOARD_SIZE + direct.x as usize
    }

    /// Converts an index to coordinates.
    fn coords_for(index: usize) -> Vec2<Direct> {
        Vec2::new((index % BOARD_SIZE) as i32, (index / BOARD_SIZE) as i32)
    }

    /// Optionally fetches the field at the given position.
    pub fn get(&self, coords: impl Into<Vec2<Direct>> + Copy) -> Option<Field> {
        if Self::in_bounds(coords) {
            Some(self[coords])
        } else {
            None
        }
    }

    /// Fetches the possible moves from a given position.
    pub fn possible_moves_from(&self, coords: impl Into<Vec2<Doubled>>) -> Vec<Move> {
        let doubled: Vec2<Doubled> = coords.into();
        Vec2::<Doubled>::DIRECTIONS
            .into_iter()
            .flat_map(|v| (1..BOARD_SIZE).map(move |n| Move::sliding(doubled, n as i32 * v)))
            .take_while(|c| self.get(c.to()).unwrap_or_default().fish() > 0)
            .collect()
    }

    /// Fetches an iterator over the fields with coordinates.
    pub fn fields(&self) -> impl Iterator<Item=(Vec2<Doubled>, Field)> {
        self.fields
            .into_iter()
            .enumerate()
            .map(|(i, f)| (Self::coords_for(i).into(), f))
    }

    /// Fetches the penguins on the board.
    pub fn penguins(&self) -> impl Iterator<Item=(Vec2<Doubled>, Team)> {
        self.fields().filter_map(|(c, f)| f.penguin().map(|p| (c, p)))
    }
}

impl<V> Index<V> for Board where V: Copy + Into<Vec2<Direct>> {
    type Output = Field;

    fn index(&self, index: V) -> &Field {
        &self.fields[Self::index_for(index)]
    }
}

impl<V> IndexMut<V> for Board where V: Copy + Into<Vec2<Direct>> {
    fn index_mut(&mut self, index: V) -> &mut Field {
        &mut self.fields[Self::index_for(index)]
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
