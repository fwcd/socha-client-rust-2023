use crate::util::{Element, SCError, SCResult};

use super::{Board, Move, Team, PENGUINS_PER_TEAM, TEAMS, Vec2, Field, Doubled};

// Ported from https://github.com/software-challenge/backend/blob/a3145a91749abb73ca5ffd426fd2a77d9a90967a/plugin/src/main/kotlin/sc/plugin2023/GameState.kt

/// The state of the game at a point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /// The game board.
    board: Board,
    /// The turn of the game.
    turn: usize,
    /// The fish per team.
    fish: [usize; TEAMS],
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
        self.start_team.opponent_if(|_| self.turn % 2 != 0)
    }

    /// Whether the given team cannot move.
    pub fn immovable(&self, team: Option<Team>) -> bool {
        let penguins: Vec<_> = self.board.penguins()
            .filter(|&(_, p)| team.is_none() || Some(p) == team)
            .collect();
        if penguins.len() == PENGUINS_PER_TEAM * team.map_or(TEAMS, |_| 1) {
            penguins
                .into_iter()
                .all(|(c, _)| c
                    .hex_neighbors()
                    .into_iter()
                    .all(|n| self.board.get(n).unwrap_or_default().fish() == 0))
        } else {
            false
        }
    }

    /// The current team.
    pub fn current_team(&self) -> Team {
        self.current_team_from_turn().opponent_if(|t| self.immovable(Some(t)))
    }

    /// The current team's fields.
    pub fn current_pieces(&self) -> impl Iterator<Item=(Vec2<Doubled>, Field)> {
        let team = self.current_team();
        self.board.fields()
            .filter(move |(_, f)| f.penguin() == Some(team))
    }

    /// Whether the current team has placed all of its penguins.
    pub fn penguins_placed(&self) -> bool {
        self.current_pieces().count() == PENGUINS_PER_TEAM
    }

    /// Fetches the possible moves.
    pub fn possible_moves(&self) -> Vec<Move> {
        if self.penguins_placed() {
            self.current_pieces()
                .flat_map(|(c, _)| self.board.possible_moves_from(c))
                .collect()
        } else {
            self.board.fields()
                .filter(|(_, f)| f.fish() == 1)
                .map(|(c, _)| Move::placing(c))
                .collect()
        }
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

    use crate::{util::Element, game::{Board, Team, State, Move, Vec2, Direct}};

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
            last_move: Some(Move::placing(Vec2::<Direct>::new(13, 5))),
            start_team: Team::One,
        });
    }
}
