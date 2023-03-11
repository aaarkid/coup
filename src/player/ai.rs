

impl Player for HumanPlayer {
    fn choose_action(&self, actions: Vec<Action>, _game_state: &GameState) -> Action {
        //display all possible actions in one line
        let coins = self.coins();
        print!("{}: You have {} coins. Possible actions are: ", self.name, coins);
        for (i, action) in actions.iter().enumerate() {
            print!("{}. {}, ", i, action);
        }
        println!();
        //read number from user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut result = input.parse::<usize>().unwrap();
        //return the action
        while result >= actions.len() {
            println!("Invalid input, please try again");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            result = input.parse::<usize>().unwrap();
        }
        actions[result].clone()
    }

    fn choose_card(&self, _game_state: &GameState) -> Character {
        let cards = self.hand();
        //display all cards in the hand in one line
        print!("{}: Your cards are: ", self.name);
        for (i, card) in cards.iter().enumerate() {
            print!("{}. {}, ", i, card);
        }
        println!();
        //read number from user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut result = input.parse::<usize>().unwrap();
        //return the card
        while result >= cards.len() {
            println!("Invalid input, please try again");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            result = input.parse::<usize>().unwrap();
        }
        cards[result]
    }

    fn possible_actions(&self, game_state: &GameState) -> Vec<Action> {
        let mut actions = vec![];
        let number_of_players = game_state.players.len();
        match game_state.phase {
            crate::phase::Phase::Action => {
                let coins = self.coins();
                if coins >= 7 {
                    for i in 0..number_of_players {
                        if i != game_state.current_player {
                            actions.push(Action::Coup(i));
                        }
                    }
                }
                if coins < 10 {
                    if coins >= 3 {
                        for i in 0..number_of_players {
                            if i != game_state.current_player {
                                actions.push(Action::Assassinate(i));
                            }
                        }
                    }
                    for i in 0..number_of_players {
                        if i != game_state.current_player {
                            actions.push(Action::Steal(i, game_state.players[i].coins().min(2)));
                        }
                    }
                    actions.push(Action::ForeignAid);
                    actions.push(Action::Income);
                    actions.push(Action::Exchange);
                }
            }
            crate::phase::Phase::Block => {
                let index = game_state.players.iter().position(|p| p.name() == self.name()).unwrap();
                let mut search = game_state.history.len() - 1;
                while !game_state.history[search].0.is_action() {
                    search -= 1;
                }
                let (action, player_index) = &game_state.history[search];

                match action {
                    Action::Assassinate(self_index) if *self_index == index => {
                        //if contessa is in hand
                        if self.hand().iter().any(|c| c == &Character::Contessa) {
                            actions.push(Action::Block(crate::action::BlockAction::Assassinate(*self_index), game_state.current_player, Some(Character::Contessa)));
                        } else {
                            actions.push(Action::Block(crate::action::BlockAction::Assassinate(*self_index), game_state.current_player, None));
                        }
                    }
                    Action::Steal(self_index, amount) if *self_index == index => {
                        if self.hand().contains(&Character::Captain) || self.hand().contains(&Character::Ambassador) {
                            if self.hand().contains(&Character::Captain) {
                                actions.push(Action::Block(crate::action::BlockAction::Stealing(*self_index, *amount), game_state.current_player, Some(Character::Captain)));
                            }
                            if self.hand().contains(&Character::Ambassador) {
                                actions.push(Action::Block(crate::action::BlockAction::Stealing(*self_index, *amount), game_state.current_player, Some(Character::Ambassador)));
                            }
                        } else {
                            actions.push(Action::Block(crate::action::BlockAction::Stealing(*self_index, *amount), game_state.current_player, None));
                        }
                    }
                    Action::ForeignAid => {
                        if self.hand().contains(&Character::Duke) {
                            actions.push(Action::Block(crate::action::BlockAction::ForeignAid(game_state.current_player), game_state.current_player, Some(Character::Duke)));
                        } else {
                            actions.push(Action::Block(crate::action::BlockAction::ForeignAid(game_state.current_player), game_state.current_player, None));
                        }
                    }
                    _ => {}
                }
            }
            crate::phase::Phase::Challenge => {
                let mut search = game_state.history.len() - 1;
                while !game_state.history[search].0.is_action_or_block() {
                    search -= 1;
                }
                let (action, player_index) = &game_state.history[search];
                let player_index = *player_index;
                match action {
                    Action::Block(block_action, player, char) => {
                        match block_action {
                            crate::action::BlockAction::Assassinate(from) => {
                                actions.push(Action::Challenge(crate::action::ChallengeAction::BlockAssassination(*from), player_index));
                            }
                            crate::action::BlockAction::Stealing(from, amount) => {
                                actions.push(Action::Challenge(crate::action::ChallengeAction::BlockStealing(*from, *amount), player_index));
                            }
                            crate::action::BlockAction::ForeignAid(from) => {
                                actions.push(Action::Challenge(crate::action::ChallengeAction::BlockForeignAid(*from), player_index));
                            }
                        }
                    }
                    Action::Exchange => {
                        actions.push(Action::Challenge(crate::action::ChallengeAction::Exchange, player_index));
                    }
                    Action::Steal(target, amount) => {
                        actions.push(Action::Challenge(crate::action::ChallengeAction::Stealing(*target, *amount), player_index));
                    }
                    Action::Assassinate(target) => {
                        actions.push(Action::Challenge(crate::action::ChallengeAction::Assassination(*target), player_index));
                    }
                    Action::Tax => {
                        actions.push(Action::Challenge(crate::action::ChallengeAction::Tax, player_index));
                    }
                    _ => {}
                }
            }
        }
        actions
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn coins(&self) -> usize {
        self.coins
    }

    fn add_coins(&mut self, coins: usize) {
        self.coins += coins;
    }

    fn lose_coins(&mut self, coins: usize) -> Result<(), String> {
        if coins <= self.coins {
            self.coins -= coins;
            Ok(())
        } else {
            Err("Not enough coins".to_string())
        }
    }

    fn hand(&self) -> &[Character] {
        &self.hand
    }

    fn add_card_to_hand(&mut self, card: Character) {
        self.hand.push(card);
    }

    fn remove_card_from_hand(&mut self, card: Character) -> Result<(), String> {
        if let Some(index) = self.hand.iter().position(|&c| c == card) {
            self.hand.remove(index);
            Ok(())
        } else {
            Err("Card not found in hand".to_string())
        }
    }
}
