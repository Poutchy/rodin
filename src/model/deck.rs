use super::{card::Card, card_color::CardColor};

pub fn deck() -> Vec<Card> {
    let mut cards = Vec::new();
    let card_colors = [
        CardColor::Red,
        CardColor::Blue,
        CardColor::White,
        CardColor::Green,
        CardColor::Cyan,
    ];

    for card_color in card_colors.iter() {
        for value in 1..=9 {
            cards.push(Card { value, card_color: *card_color });
        }
    }

    cards
}

