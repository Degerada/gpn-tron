use std::collections::HashMap;

use crate::parser::MessageTypes;

pub struct GameState {
    pub(crate) grid: Vec<Vec<usize>>,
    pub(crate) my_id: usize,
    pub(crate) players: HashMap<usize, PlayerState>,
    pub(crate) gridsize: usize,
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct PlayerState {
    pub id: usize,
    pub alive: bool,
    pub position: Point,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GameState {
    pub(crate) fn new(map_width: usize, map_height: usize, my_id: usize, gridsize: usize) -> Self {
        Self {
            grid: vec![vec![0; map_width]; map_height],
            my_id,
            players: HashMap::new(),
            gridsize,
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

                self.grid[*pos_y as usize][*pos_x as usize] = *player_id;
            }
            MessageTypes::Player { player_id } => {
                if !self.players.contains_key(player_id) {
                    self.players.insert(
                        *player_id,
                        PlayerState {
                            id: *player_id,
                            alive: true,
                            position: Point { x: 0, y: 0 },
                        },
                    );
                }
            }
            MessageTypes::Die { player_id } => {
                self.players.get_mut(&player_id).unwrap().alive = false;
                for row in self.grid.iter_mut() {
                    for tile in row.iter_mut() {
                        if *tile == *player_id {
                            *tile = 0;
                        }
                    }
                }
            }
            MessageTypes::Tick => {}
            _ => {}
        }
    }
}
