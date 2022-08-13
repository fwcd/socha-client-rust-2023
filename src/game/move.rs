// Port of https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/Move.kt

use crate::util::{Element, SCError, SCResult};

use super::Vec2;

/// A game move.
pub struct Move {
    from: Option<Vec2>,
    to: Vec2,
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
