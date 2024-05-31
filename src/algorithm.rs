use crate::gamestate::{Direction, GameState};

pub struct Algorithm<'a> {
    game_state: &'a GameState,
}

impl<'a> Algorithm<'a> {
    pub fn new(game_state: &'a GameState) -> Self {
        Self { game_state }
    }

    pub fn calculate_next_move(self) -> Direction {
        if self.is_direction_valid(Direction::Up) {
            return Direction::Up;
        }
        if self.is_direction_valid(Direction::Left) {
            return Direction::Left;
        }
        if self.is_direction_valid(Direction::Right) {
            return Direction::Right;
        }
        if self.is_direction_valid(Direction::Down) {
            return Direction::Down;
        }

        return Direction::Up;
    }

    fn is_direction_valid(&self, direction: Direction) -> bool {
        let game_state = &self.game_state;
        let players = &game_state.players;
        let myself = players.get(&self.game_state.my_id).clone().unwrap();
        let my_pos = &myself.position;

        return match direction {
            Direction::Up => {
                let next_y = if my_pos.y == 0 {
                    game_state.gridsize - 1
                } else {
                    my_pos.y - 1
                };
                self.game_state.grid[next_y][my_pos.x] == 0
            }
            Direction::Down => {
                let next_y = if my_pos.y == self.game_state.gridsize - 1 {
                    0
                } else {
                    my_pos.y + 1
                };
                self.game_state.grid[next_y][my_pos.x] == 0
            }
            Direction::Left => {
                let next_x = if my_pos.x == 0 {
                    game_state.gridsize - 1
                } else {
                    my_pos.x - 1
                };

                self.game_state.grid[my_pos.y][next_x] == 0
            }
            Direction::Right => {
                let next_x = if my_pos.x == self.game_state.gridsize - 1 {
                    0
                } else {
                    my_pos.x + 1
                };
                self.game_state.grid[my_pos.y][next_x] == 0
            }
        };
    }
}
