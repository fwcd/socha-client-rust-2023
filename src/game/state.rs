use crate::util::{Element, SCError, SCResult};

use super::{Board, Move, Team};

/// The state of the game at a point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The turn of the game.
    turn: usize,
    /// The fish per team.
    fish: Vec<usize>,
    /// The most recent move.
    last_move: Option<Move>,
    /// The starting team.
    start_team: Option<Team>,
}


impl TryFrom<&Element> for State {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(State {
            board: elem.child_by_name("board")?.try_into()?,
            turn: elem.attribute("turn")?.parse()?,
            fish: elem.child_by_name("fishes")?.childs_by_name("int").map(|c| Ok(c.content().parse()?)).collect::<SCResult<_>>()?,
            last_move: elem.child_by_name("lastMove").ok().and_then(|m| m.try_into().ok()),
            start_team: elem.child_by_name("startTeam").ok().and_then(|t| t.content().parse().ok()),
        })
    }
}

// TODO: Add test
