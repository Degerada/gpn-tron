use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use gamestate::{Direction, GameState};
use parser::MessageTypes;

mod gamestate;
mod parser;

fn main() {
    let mut gamestate = GameState {
        grid: vec![],
        my_id: 0,
        players: Default::default(),
        next_move: Direction::Up,
    };
    let playername = env!("username");
    let password = env!("password");
    let url = "gpn-tron.duckdns.org:4000";

    // Connect
    let mut connection = TcpStream::connect(url).unwrap();
    let mut buf_reader = BufReader::new(connection.try_clone().unwrap());

    let mut buffer = String::from("");

    loop {
        // Read latest data
        buffer = String::new();
        buf_reader.read_line(&mut buffer).unwrap();

        // Parse messages from server
        let read_messages = parser::parse_read_from_buffer(buffer.clone());

        // Act upon message
        read_messages.iter().for_each(|message| match message {
            MessageTypes::Motd => {
                // Send Join
                let mut join_message = generate_join_message(playername, password);
                println!("Sending join message {}", join_message);
                connection.write_all(join_message.as_bytes()).unwrap();
            }
            MessageTypes::Join {
                username: _username,
                password: _password,
            } => {}
            MessageTypes::Error {
                error_text: _errorText,
            } => {}
            MessageTypes::Game {
                map_width,
                map_height,
                player_id,
            } => gamestate = GameState::new(*map_width, *map_height, *player_id),
            MessageTypes::Pos {
                player_id,
                pos_x,
                pos_y,
            } => gamestate.process(message),
            MessageTypes::Player {
                player_id,
                player_name,
            } => gamestate.process(message),
            MessageTypes::Tick => {
                gamestate.process(&message);
                let move_message = generate_move_message(&gamestate);
                connection.write(move_message.as_bytes()).unwrap();
            }
            MessageTypes::Die { player_id } => gamestate.process(&message),
            MessageTypes::Move { direction } => {}
            _ => {}
        });

        connection.flush().unwrap();
        std::thread::sleep(Duration::from_millis(30));
    }
}

fn generate_join_message(username: &str, password: &str) -> String {
    return format!("join|{}|{}\n", username, password);
}

fn generate_move_message(game_state: &GameState) -> &'static str {
    return match game_state.next_move {
        Direction::Up => "move|up\n",
        Direction::Down => "move|down\n",
        Direction::Left => "move|left\n",
        Direction::Right => "move|right\n",
    };
}
