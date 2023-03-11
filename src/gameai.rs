use crate::action::{Action, BlockAction, ChallengeAction};
use crate::character::Character;
use crate::game::GameState;

#[derive(Debug)]
pub struct GameStateAI {
    pub players_hands: Vec<usize>,
    pub deck_size: usize,
    pub revealed_characters: [Option<Character>; 6],
    pub history: Vec<(Action, usize)>,
    pub current_player: usize,
    pub phase: Phase,
}

impl GameStateAI {
    pub fn from_gamestate(gamestate: &GameState) -> GameStateAI {
        GameStateAI {
            players_hands: gamestate.players.iter().map(|player| player.hand.len()).collect(),
            deck_size: gamestate.deck.len(),
            revealed_characters: gamestate.revealed_characters.clone(),
            history: gamestate.history.clone(),
            current_player: gamestate.current_player,
            phase: gamestate.phase,
        }
    }
}