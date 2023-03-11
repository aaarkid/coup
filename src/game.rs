use rand::seq::SliceRandom;

use crate::character::Character;
use crate::player::Player;
use crate::action::{Action, self, BlockAction};
use crate::phase::Phase;

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Box<dyn Player>>,
    pub deck: Vec<Character>,
    pub revealed_characters: [Option<Character>; 6],
    pub history: Vec<(Action, usize)>,
    pub current_player: usize,
    pub phase: Phase,
}

impl GameState {
    pub fn new(players: Vec<Box<dyn Player>>) -> GameState {
        GameState {
            players,
            deck: Character::create_deck(),
            revealed_characters: [None; 6],
            history: vec![],
            current_player: 0,
            phase: Phase::Action,
        }
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    pub fn resolve_challenge(&mut self, action: Action, challenger: usize, blocker: Option<usize>) -> bool {
        //is the card for the responsible action in the player's hand?
        let challenge_successful = match action {
            Action::Tax => {
                //if player has duke, challenge is successful
                self.players[self.current_player].hand().contains(&Character::Duke)
            },
            Action::Assassinate(_) => {
                //if player has assassin, challenge is successful
                self.players[self.current_player].hand().contains(&Character::Assassin)
            },
            Action::Steal(_, _) => {
                //if player has captain, challenge is successful
                self.players[self.current_player].hand().contains(&Character::Captain)
            },
            Action::Exchange => {
                //if player has ambassador, challenge is successful
                self.players[self.current_player].hand().contains(&Character::Ambassador)
            },
            Action::Block(block_action, _, _) => {
                let blocker = blocker.unwrap();
                match block_action {
                    BlockAction::ForeignAid(_) => {
                        //if player has duke, challenge is successful
                        self.players[blocker].hand().contains(&Character::Duke)
                    },
                    action::BlockAction::Assassinate(_) => {
                        //if player has contessa, challenge is successful
                        self.players[blocker].hand().contains(&Character::Contessa)
                    },
                    action::BlockAction::Stealing(_, _) => {
                        //if player has captain or ambassador, challenge is successful
                        self.players[blocker].hand().contains(&Character::Captain) || self.players[blocker].hand().contains(&Character::Ambassador)
                    },
                }
            },
            _ => {true},
        };

        if challenge_successful {
            //if challenge is successful, the challenger loses an influence
            let card = self.players[challenger].choose_card(&self);
            let _ = self.players[challenger].remove_card_from_hand(card);
            self.check_elimination(challenger)
        } else {
            if blocker.is_none() {
                //if challenge is unsuccessful, the player loses an influence
                let card = self.players[self.current_player].choose_card(&self);
                let _ = self.players[self.current_player].remove_card_from_hand(card);
                self.check_elimination(self.current_player)
            } else {
                //if challenge is unsuccessful, the blocker loses an influence
                let card = self.players[blocker.unwrap()].choose_card(&self);
                let _ = self.players[blocker.unwrap()].remove_card_from_hand(card);
                self.check_elimination(blocker.unwrap())
            }
        }

        true
    }

    pub fn check_elimination(&mut self, index: usize) {
        if self.players[index].hand().len() == 0 {
            println!("Player {} eliminated!", self.players[index].name());
            self.players.remove(index);
            if self.current_player >= index {
                self.current_player -= 1;
            }
        }
    }

    pub fn game_over(&self) -> bool {
        self.players.len() == 1
    }

    pub fn winner(&self) -> Option<&Box<dyn Player>> {
        if self.game_over() {
            Some(&self.players[0])
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        while !self.game_over() {
            let current_player = self.current_player;

            // ACTION PHASE
            self.phase = Phase::Action;
            let actions = self.players[current_player].possible_actions(self);
            let action = self.players[current_player].choose_action(actions, self);
            self.history.push((action.clone(), current_player));
            let mut challenged = false;
            let mut blocked = false;

            // CHALLENGE PHASE
            self.phase = Phase::Challenge;
            //loop all players except current player
            for i in 0..self.players.len() {
                if i != current_player {
                    let mut challenges = self.players[i].possible_actions(self);
                    if challenges.len() == 0 {
                        continue;
                    }
                    challenges.push(Action::Pass);
                    let challenge = self.players[i].choose_action(challenges, self);
                    self.history.push((challenge.clone(), i));
                    
                    // if Pass was picked, continue
                    if let Action::Pass = challenge {
                        continue;
                    }
                    challenged = self.resolve_challenge(action.clone(), i, None);
                    if challenged {
                        break;
                    }
                }
            }
            // BLOCK PHASE
            if !challenged {
                //loop all players except current player
                for i in 0..self.players.len() {
                    if i != current_player {
                        let mut block_challenged = false;
                        self.phase = Phase::Block;
                        let mut blocks = self.players[i].possible_actions(self);
                        println!("Player {} has {} blocks", i, blocks.len());
                        if blocks.len() == 0 {
                            continue;
                        }
                        blocks.push(Action::Pass);
                        let block = self.players[i].choose_action(blocks, self);
                        self.history.push((block.clone(), i));
                        
                        // if Pass was picked, continue
                        if let Action::Pass = block {
                            continue;
                        }

                        //CHALLENGE PHASE
                        self.phase = Phase::Challenge;
                        //loop all players except player who blocked
                        for j in 0..self.players.len() {
                            if j != i {
                                let mut challenges = self.players[j].possible_actions(self);
                                if challenges.len() == 0 {
                                    continue;
                                }
                                challenges.push(Action::Pass);
                                let challenge = self.players[j].choose_action(challenges, self);
                                self.history.push((challenge.clone(), j));
                                
                                // if Pass was picked, continue
                                if let Action::Pass = challenge {
                                    continue;
                                }
                                block_challenged = self.resolve_challenge(block.clone(), j, Some(i));
                                if block_challenged {
                                    break;
                                }
                            }
                        }

                        if !block_challenged {
                            blocked = true;
                        }
                    }
                }
            }

            if !challenged && !blocked {
                match action {
                    Action::Income => {
                        self.players[current_player].add_coins(1);
                    },
                    Action::ForeignAid => {
                        self.players[current_player].add_coins(2);
                    },
                    Action::Coup(target) => {
                        self.players[current_player].lose_coins(7);
                        let card = self.players[target].choose_card(self);
                        self.players[target].remove_card_from_hand(card);
                        self.check_elimination(target);
                    },
                    Action::Tax => {
                        self.players[current_player].add_coins(3);
                    },
                    Action::Assassinate(target) => {
                        self.players[current_player].lose_coins(3);
                        let card = self.players[target].choose_card(self);
                        self.players[target].remove_card_from_hand(card);
                        self.check_elimination(target);
                    },
                    Action::Exchange => {
                        let card1 = self.deck.pop().unwrap();
                        let card2 = self.deck.pop().unwrap();
                        self.players[current_player].add_card_to_hand(card1);
                        self.players[current_player].add_card_to_hand(card2);
                        let card1 = self.players[current_player].choose_card(self);
                        let card2 = self.players[current_player].choose_card(self);
                        self.players[current_player].remove_card_from_hand(card1);
                        self.players[current_player].remove_card_from_hand(card2);
                        self.deck.shuffle(&mut rand::thread_rng());
                    },
                    Action::Steal(target, amount) => {
                        self.players[current_player].add_coins(amount);
                        self.players[target].lose_coins(amount);
                    },
                    _ => {},
                }
            }

            // check if game is over
            if self.game_over() {
                //announce winner
                println!("{} wins!", self.winner().unwrap().name());
            } else {
                self.next_player();
            }
        }
    }
}

pub fn start_game(state: &mut GameState) {
    state.players.shuffle(&mut rand::thread_rng());

    state.deck.shuffle(&mut rand::thread_rng());

    // Distribute two cards to each player
    for player in state.players.iter_mut() {
        let card1 = state.deck.pop().unwrap();
        let card2 = state.deck.pop().unwrap();
        player.add_card_to_hand(card1);
        player.add_card_to_hand(card2);
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for (_i, player) in self.players.iter().enumerate() {
            s.push_str(&format!("{} has {} coins and {:?} cards in hand. ", player.name(), player.coins(), player.hand()));
        }
        write!(f, "{}", s)
    }
}
