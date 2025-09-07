use super::card_color::CardColor;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub value: i32,
    pub card_color: CardColor
}

impl Eq for Card {
}


impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
