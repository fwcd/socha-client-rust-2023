use crate::util::{Element, SCError, SCResult};

use super::{Board, Move, Team};

// Ported from https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/GameState.kt

/// The state of the game at a point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The turn of the game.
    turn: usize,
    /// The fish per team.
    fish: [usize; 2],
    /// The most recent move.
    last_move: Option<Move>,
    /// The starting team.
    start_team: Team,
}

impl State {
    /// Fetches the board.
    pub fn board(&self) -> &Board { &self.board }

    /// Fetches the turn of the game.
    pub fn turn(&self) -> usize { self.turn }

    /// Fetches the fish for the given team.
    pub fn fish(&self, team: Team) -> usize { self.fish[team.index()] }

    /// Fetches the most recent move.
    pub fn last_move(&self) -> Option<Move> { self.last_move }

    /// Fetches the starting team.
    pub fn start_team(&self) -> Team { self.start_team }

    /// The current team, computed from the starting team and the turn.
    pub fn current_team_from_turn(&self) -> Team {
        if self.turn % 2 == 0 {
            self.start_team
        } else {
            self.start_team.opponent()
        }
    }

    /// The current team.
    pub fn current_team(&self) -> Team {
        // TODO
        self.current_team_from_turn()
    }

    /// Fetches the possible moves.
    pub fn possible_moves(&self) -> Vec<Move> {
        todo!()
    }
}

impl TryFrom<&Element> for State {
    type Error = SCError;

    fn try_from(elem: &Element) -> SCResult<Self> {
        Ok(State {
            board: elem.child_by_name("board")?.try_into()?,
            turn: elem.attribute("turn")?.parse()?,
            fish: elem.child_by_name("fishes")?
                .childs_by_name("int").map(|c| Ok(c.content().parse()?))
                .collect::<SCResult<Vec<usize>>>()?
                .try_into()
                .map_err(|e| SCError::from(format!("State has wrong number of fish teams: {:?}", e)))?,
            last_move: elem.child_by_name("lastMove").ok().and_then(|m| m.try_into().ok()),
            start_team: elem.child_by_name("startTeam")?.content().parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{util::Element, game::{Board, Team, State, Move, Vec2}};

    #[test]
    fn test_parsing_state() {
        assert_eq!(State::try_from(&Element::from_str(r#"
            <state class="state" turn="1">
                <startTeam>ONE</startTeam>
                <board>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                    <list>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                        <field>0</field>
                    </list>
                </board>
                <lastMove>
                    <to x="13" y="5"/>
                </lastMove>
                <fishes>
                    <int>1</int>
                    <int>0</int>
                </fishes>
            </state>
        "#).unwrap()).unwrap(), State {
            board: Board::EMPTY,
            turn: 1,
            fish: [1, 0],
            last_move: Some(Move::placing(Vec2::new(13, 5))),
            start_team: Team::One,
        });
    }
}
