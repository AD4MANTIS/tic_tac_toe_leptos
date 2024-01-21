use super::{field::Fields, symbol::PlayerSymbol};

pub fn has_won(player: PlayerSymbol, fields: Fields) -> bool {
    fields
        .all_win_lines()
        .into_iter()
        .any(|row| row.into_iter().all(|field| field == Some(player)))
}
