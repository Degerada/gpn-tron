use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use gamestate::GameState;
use parser::MessageTypes;

use crate::algorithm::Algorithm;
use crate::parser::Command;

mod algorithm;
mod gamestate;
mod parser;

fn main() {
    let mut gamestate = GameState {
        grid: vec![],
        my_id: 0,
        players: Default::default(),
    };

    let username: String = env!("username").to_owned();
    let password: String = env!("password").to_owned();
    let url = "gpn-tron.duckdns.org:4000";

    // Connect
    let mut connection = TcpStream::connect(url).unwrap();
    let mut buf_reader = BufReader::new(connection.try_clone().unwrap());

    loop {
        // Read latest data
        let mut buffer = String::new();
        buf_reader.read_line(&mut buffer).unwrap();

        // Parse messages from server
        let read_messages = parser::parse_read_from_buffer(buffer.clone());

        // Act upon message
        for message in read_messages {
            match message {
                MessageTypes::Motd => {
                    // Send Join
                    let join_message = Command::Join {
                        username: username.clone(),
                        password: password.clone(),
                    }
                    .as_str();
                    println!("Sending join message {}", join_message);
                    connection.write_all(join_message.as_bytes()).unwrap();
                }
                MessageTypes::Error { .. } => {}
                MessageTypes::Game {
                    map_width,
                    map_height,
                    player_id,
                } => gamestate = GameState::new(map_width, map_height, player_id),
                MessageTypes::Pos { .. }
                | MessageTypes::Player { .. }
                | MessageTypes::Die { .. } => gamestate.process(&message),
                MessageTypes::Tick => {
                    let algorithm = Algorithm::new(&gamestate);
                    let direction = algorithm.calculate_next_move();
                    let move_message = Command::Move { direction }.as_str();
                    connection.write(move_message.as_bytes()).unwrap();
                }
            }
        }

        connection.flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }
}
