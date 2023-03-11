#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Action,
    Block,
    Challenge,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Phase::Action => write!(f, "Action Phase"),
            Phase::Block => write!(f, "Block Phase"),
            Phase::Challenge => write!(f, "Challenge Phase"),
        }
    }
}