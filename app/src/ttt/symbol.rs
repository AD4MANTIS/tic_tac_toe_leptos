#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerSymbol {
    X,
    O,
}

impl ToString for PlayerSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::X => "X",
            Self::O => "O",
        }
        .to_owned()
    }
}
