use rusty_machine::prelude::Matrix;

use crate::action::{Action, BlockAction, ChallengeAction};
use crate::character::Character;
use crate::game::GameState;
use crate::phase::Phase;

#[derive(Debug)]
pub struct GameStateAI {
    pub players_lives: Vec<usize>,
    pub players_coins: Vec<usize>,
    pub revealed_characters: Vec<Option<Character>>,
    pub cards_in_hand: Vec<Character>,
}


impl GameStateAI {
    pub fn from_gamestate(gamestate: &GameState, player_name: String) -> GameStateAI {
        GameStateAI {
            players_lives: gamestate.players.iter().map(|player| player.hand().len()).collect(),
            players_coins: gamestate.players.iter().map(|player| player.coins()).collect(),
            revealed_characters: gamestate.revealed_characters.clone(),
            cards_in_hand: gamestate.players.iter().find(|player| player.name() == player_name).unwrap().hand().clone(),
        }
    }
}