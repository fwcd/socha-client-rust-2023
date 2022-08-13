// Port of https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Move.kt

use std::fmt;

use crate::util::{Element, SCError, SCResult};

use super::Vec2;

/// A game move.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    from: Option<Vec2>,
    to: Vec2,
}

impl Move {
    /// Creates a new move from the given field to the given field.
    pub fn new(from: Option<Vec2>, to: Vec2) -> Self {
        Self { from, to }
    }

    /// Convenience constructor for creating a move of a penguin between two fields.
    pub fn between(from: Vec2, to: Vec2) -> Self {
        Self::new(Some(from), to)
    }

    /// Convenience constructor for creating a move placing a penguin.
    pub fn placing(pos: Vec2) -> Self {
        Self::new(None, pos)
    }

    /// The source position if this is a move of an existing penguin, otherwise None.
    pub fn from(self) -> Option<Vec2> { self.from }

    /// The target position of the penguin.
    pub fn to(self) -> Vec2 { self.to }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(from) = self.from {
            write!(f, "{} -> {}", from, self.to)
        } else {
            write!(f, "-> {}", self.to)
        }
    }
}

impl TryFrom<&Element> for Move {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(Self {
            from: elem.child_by_name("from").ok().map(Vec2::try_from).transpose()?,
            to: elem.child_by_name("to")?.try_into()?,
        })
    }
}

impl From<Move> for Element {
    fn from(m: Move) -> Self {
        Element::new("data")
            .attribute("class", "move")
            .option_child(m.from.map(|v| Element::new("from").attribute("x", v.x).attribute("y", v.y)))
            .child(Element::new("to").attribute("x", m.to.x).attribute("y", m.to.y))
            .build()
    }
}
