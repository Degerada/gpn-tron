use std::collections::HashMap;

use crate::parser::MessageTypes;

pub struct GameState {
    pub(crate) grid: Vec<Vec<u8>>,
    pub(crate) my_id: u8,
    pub(crate) players: HashMap<u8, PlayerState>,
}

pub struct Point {
    x: u8,
    y: u8,
}

pub struct PlayerState {
    id: u8,
    alive: bool,
    position: Point,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GameState {
    pub(crate) fn new(map_width: u8, map_height: u8, my_id: u8) -> Self {
        Self {
            grid: vec![vec![0; map_width as usize]; map_height as usize],
            my_id,
            players: HashMap::new(),
        }
    }

    pub(crate) fn process(&mut self, message: &MessageTypes) {
        match message {
            MessageTypes::Pos {
                player_id,
                pos_x,
                pos_y,
            } => {
                if !self.players.contains_key(&player_id) {
                    self.players.insert(
                        *player_id,
                        PlayerState {
                            id: *player_id,
                            alive: true,
                            position: Point {
                                x: *pos_x,
                                y: *pos_y,
                            },
                        },
                    );
                } else {
                    self.players.get_mut(&player_id).unwrap().position = Point {
                        x: *pos_x,
                        y: *pos_y,
                    }
                }

                // FIXME: Implement this shit
                // self.grid.get_mut(pos_y).unwrap()
            }
            MessageTypes::Player { .. } => {}
            MessageTypes::Die { player_id } => {
                self.players.get_mut(&player_id).unwrap().alive = false;
            }
            MessageTypes::Tick => {}
            _ => {}
        }
    }
}
