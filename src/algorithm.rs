use rand::{RngCore, thread_rng};

use crate::gamestate::{Direction, GameState};

pub struct Algorithm<'a> {
    game_state: &'a GameState,
}

impl<'a> Algorithm<'a> {
    pub fn new(game_state: &'a GameState) -> Self {
        Self { game_state }
    }

    pub fn calculate_next_move(self) -> Direction {
        let rng_num = thread_rng().next_u32();

        if rng_num % 4 == 0 {
            return Direction::Up;
        }

        return match rng_num % 4 {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Up,
        };
    }

    fn is_direction_valid(self, direction: Direction) -> bool {
        let game_state = &self.game_state;
        let players = &game_state.players;
        let myself = players.get(&self.game_state.my_id).clone().unwrap();
        let my_pos = &myself.position;

        return match direction {
            Direction::Up => self.game_state.grid[my_pos.y - 1][my_pos.x] == 0,
            Direction::Down => self.game_state.grid[my_pos.y + 1][my_pos.x] == 0,
            Direction::Left => self.game_state.grid[my_pos.y][my_pos.x - 1] == 0,
            Direction::Right => self.game_state.grid[my_pos.y][my_pos.x + 1] == 0,
        };
    }
}
