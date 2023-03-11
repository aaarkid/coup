#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Character {
    Duke,
    Assassin,
    Captain,
    Ambassador,
    Contessa,
}

impl Character {
    pub fn all() -> Vec<Character> {
        vec![Character::Duke, Character::Assassin, Character::Captain, Character::Ambassador, Character::Contessa]
    }

    pub fn create_deck() -> Vec<Character> {
        let mut deck = Vec::new();
        for _ in 0..3 {
            deck.push(Character::Duke);
            deck.push(Character::Assassin);
            deck.push(Character::Captain);
            deck.push(Character::Ambassador);
            deck.push(Character::Contessa);
        }
        deck
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Character::Duke => write!(f, "Duke"),
            Character::Assassin => write!(f, "Assassin"),
            Character::Captain => write!(f, "Captain"),
            Character::Ambassador => write!(f, "Ambassador"),
            Character::Contessa => write!(f, "Contessa"),
        }
    }
}