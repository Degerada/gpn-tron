use crate::gamestate::Direction;

pub enum MessageTypes {
    Motd,
    Error {
        error_text: String,
    },
    Game {
        map_width: u8,
        map_height: u8,
        player_id: u8,
    },
    Pos {
        player_id: u8,
        pos_x: u8,
        pos_y: u8,
    },
    Player {
        player_id: u8,
        player_name: String,
    },
    Tick,
    Die {
        player_id: u8,
    },
}

pub enum Command {
    Move { direction: Direction },
    Join { username: String, password: String },
}

impl Command {
    pub fn as_str(self: &Self) -> String {
        return match self {
            Command::Move { direction } => match direction {
                Direction::Up => "move|up\n".to_owned(),
                Direction::Down => "move|down\n".to_owned(),
                Direction::Left => "move|left\n".to_owned(),
                Direction::Right => "move|right\n".to_owned(),
            },
            Command::Join { username, password } => {
                format!("join|{}|{}\n", username, password)
            }
        };
    }
}

pub fn parse_read_from_buffer(buffer: String) -> Vec<MessageTypes> {
    let messages: Vec<&str> = buffer.split('\n').collect();
    let mut parsed_messages: Vec<MessageTypes> = Vec::new();

    messages.iter().for_each(|message| {
        let message_components: Vec<&str> = message.split("|").collect();
        let message_type = &message_components[0];
        println!("Parsing message {}", message);

        match *message_type {
            "game" => parsed_messages.push(MessageTypes::Game {
                map_width: message_components[1].parse().unwrap(),
                map_height: message_components[2].parse().unwrap(),
                player_id: message_components[3].parse().unwrap(),
            }),
            "pos" => parsed_messages.push(MessageTypes::Pos {
                player_id: message_components[1].parse().unwrap(),
                pos_x: message_components[2].parse().unwrap(),
                pos_y: message_components[3].parse().unwrap(),
            }),
            "player" => parsed_messages.push(MessageTypes::Player {
                player_id: message_components[1].parse().unwrap(),
                player_name: message_components[2].parse().unwrap(),
            }),
            "tick" => parsed_messages.push(MessageTypes::Tick),
            "die" => parsed_messages.push(MessageTypes::Die {
                player_id: message_components[1].parse().unwrap(),
            }),
            "motd" => parsed_messages.push(MessageTypes::Motd),
            "error" => {
                println!("Received this error: {}", message_components[1])
            }
            _ => {
                println!("Skipping this message {}", message_type)
            }
        }
    });

    return parsed_messages;
}
