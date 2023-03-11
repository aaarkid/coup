use player::Player;
use game::GameState;

use crate::player::{HumanPlayer, ai::AIPlayer};

mod game;
mod gameai;
mod player;
mod character;
mod action;
mod phase;

fn main() {
    //ask how many players
    let mut num_players = String::new();
    std::io::stdin().read_line(&mut num_players).expect("Failed to read line");
    let num_players: usize = num_players.trim_end().parse().expect("Please type a number!");
    //ask for names
    let mut players: Vec<Box<(dyn Player + 'static)>> = Vec::new();
    for _ in 0..num_players {
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim_end();
        players.push(Box::new(HumanPlayer::new(name)));
    }
    //create an ai player
    //players.push(Box::new(AIPlayer::new("AI".to_string())));

    let mut game = GameState::new(players);
    game::start_game(&mut game);
    println!("{}", game);
    
    game.run();
}
