use crate::character::Character;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Income,
    ForeignAid,
    Tax,
    Coup(usize),
    Assassinate(usize),
    Steal(usize, usize), //target, coins
    Exchange,
    Block(BlockAction, usize, Option<Character>), //BlockAction, player to be blocked, character
    Challenge(ChallengeAction, usize),
    Pass
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlockAction {
    Assassinate(usize), //from
    Stealing(usize, usize), //from
    ForeignAid(usize), //from
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChallengeAction {
    BlockAssassination(usize),
    BlockStealing(usize, usize),
    BlockForeignAid(usize),
    Exchange,
    Stealing(usize, usize),
    Assassination(usize),
    Tax,
}

impl std::fmt::Display for ChallengeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChallengeAction::BlockAssassination(target) => write!(f, "BlockAssassination on player {}", target),
            ChallengeAction::BlockStealing(target, coins) => write!(f, "BlockStealing on player {} with {} coins", target, coins),
            ChallengeAction::BlockForeignAid(target) => write!(f, "BlockForeignAid on player {}", target),
            ChallengeAction::Exchange => write!(f, "Exchange"),
            ChallengeAction::Stealing(target, coins) => write!(f, "Stealing on player {} with {} coins", target, coins),
            ChallengeAction::Assassination(target) => write!(f, "Assassination on player {}", target),
            ChallengeAction::Tax => write!(f, "Tax"),
        }
    }
}

impl std::fmt::Display for BlockAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockAction::Assassinate(target) => write!(f, "Assassinate on player {}", target),
            BlockAction::Stealing(target, coins) => write!(f, "Stealing on player {} with {} coins", target, coins),
            BlockAction::ForeignAid(target) => write!(f, "ForeignAid on player {}", target),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Income => write!(f, "Income"),
            Action::ForeignAid => write!(f, "Foreign Aid"),
            Action::Tax => write!(f, "Tax"),
            Action::Coup(target) => write!(f, "Coup on player {}", target),
            Action::Assassinate(target) => write!(f, "Assassinate player {}", target),
            Action::Steal(target, _) => write!(f, "Steal from player {}", target),
            Action::Exchange => write!(f, "Exchange"),
            Action::Block(block_action, target, _char) => write!(f, "Block {} on player {}.", block_action, target),
            Action::Challenge(challenge_action, target) => write!(f, "Challenge {} on player {}", challenge_action, target),
            Action::Pass => write!(f, "Pass"),
        }
    }
}

impl Action {
    pub fn is_action(&self) -> bool {
        match self {
            Action::Income => true,
            Action::ForeignAid => true,
            Action::Tax => true,
            Action::Coup(_) => true,
            Action::Assassinate(_) => true,
            Action::Steal(_, _) => true,
            Action::Exchange => true,
            Action::Block(_, _, _) => false,
            Action::Challenge(_, _) => false,
            Action::Pass => false,
        }
    }

    pub fn is_action_or_block(&self) -> bool {
        match self {
            Action::Income => true,
            Action::ForeignAid => true,
            Action::Tax => true,
            Action::Coup(_) => true,
            Action::Assassinate(_) => true,
            Action::Steal(_, _) => true,
            Action::Exchange => true,
            Action::Block(_, _, _) => true,
            Action::Challenge(_, _) => false,
            Action::Pass => false,
        }
    }

    pub fn all_3players(pos0: usize, pos1: usize, pos2:usize) -> Vec<Action> {
        vec![
            Action::Income,
            Action::ForeignAid,
            Action::Tax,
            Action::Coup(pos1),
            Action::Coup(pos2),
            Action::Assassinate(pos1),
            Action::Assassinate(pos2),
            Action::Steal(pos1, 2),
            Action::Steal(pos2, 2),
            Action::Steal(pos1, 1),
            Action::Steal(pos2, 1),
            Action::Steal(pos1, 0),
            Action::Steal(pos2, 0),
            Action::Exchange,
            Action::Block(BlockAction::Assassinate(pos1), pos0, None),
            Action::Block(BlockAction::Assassinate(pos2), pos0, None),
            Action::Block(BlockAction::Stealing(pos1, 2), pos0, None),
            Action::Block(BlockAction::Stealing(pos2, 2), pos0, None),
            Action::Block(BlockAction::Stealing(pos1, 1), pos0, None),
            Action::Block(BlockAction::Stealing(pos2, 1), pos0, None),
            Action::Block(BlockAction::Stealing(pos1, 0), pos0, None),
            Action::Block(BlockAction::Stealing(pos2, 0), pos0, None),
            Action::Block(BlockAction::ForeignAid(pos1), pos0, None),
            Action::Block(BlockAction::ForeignAid(pos2), pos0, None),
            Action::Block(BlockAction::Assassinate(pos1), pos0, Some(Character::Contessa)),
            Action::Block(BlockAction::Assassinate(pos2), pos0, Some(Character::Contessa)),
            Action::Block(BlockAction::Stealing(pos1, 2), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos2, 2), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos1, 1), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos2, 1), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos1, 0), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos2, 0), pos0, Some(Character::Captain)),
            Action::Block(BlockAction::Stealing(pos1, 2), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::Stealing(pos2, 2), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::Stealing(pos1, 1), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::Stealing(pos2, 1), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::Stealing(pos1, 0), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::Stealing(pos2, 0), pos0, Some(Character::Ambassador)),
            Action::Block(BlockAction::ForeignAid(pos1), pos0, Some(Character::Duke)),
            Action::Block(BlockAction::ForeignAid(pos2), pos0, Some(Character::Duke)),
            Action::Challenge(ChallengeAction::BlockForeignAid(pos0), pos1),
            Action::Challenge(ChallengeAction::BlockForeignAid(pos0), pos2),
            Action::Challenge(ChallengeAction::BlockForeignAid(pos1), pos2),
            Action::Challenge(ChallengeAction::BlockForeignAid(pos2), pos1),
            Action::Challenge(ChallengeAction::BlockAssassination(pos0), pos1),
            Action::Challenge(ChallengeAction::BlockAssassination(pos0), pos2),
            Action::Challenge(ChallengeAction::BlockAssassination(pos1), pos2),
            Action::Challenge(ChallengeAction::BlockAssassination(pos2), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 2), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 2), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos1, 2), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos2, 2), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 1), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 1), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos1, 1), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos2, 1), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 0), pos1),
            Action::Challenge(ChallengeAction::BlockStealing(pos0, 0), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos1, 0), pos2),
            Action::Challenge(ChallengeAction::BlockStealing(pos2, 0), pos1),
            Action::Challenge(ChallengeAction::Exchange, pos1),
            Action::Challenge(ChallengeAction::Exchange, pos2),
            Action::Challenge(ChallengeAction::Tax, pos1),
            Action::Challenge(ChallengeAction::Tax, pos2),
            Action::Challenge(ChallengeAction::Assassination(pos0), pos1),
            Action::Challenge(ChallengeAction::Assassination(pos0), pos2),
            Action::Challenge(ChallengeAction::Assassination(pos1), pos2),
            Action::Challenge(ChallengeAction::Assassination(pos2), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 2), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 2), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos1, 2), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos2, 2), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 1), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 1), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos1, 1), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos2, 1), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 0), pos1),
            Action::Challenge(ChallengeAction::Stealing(pos0, 0), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos1, 0), pos2),
            Action::Challenge(ChallengeAction::Stealing(pos2, 0), pos1),
        ]
    }
}