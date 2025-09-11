use super::card::Card;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Play {
    pub cards: Vec<Card>,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut selfres = 0;
        for selfcard in self.cards.iter() {
            selfres = selfres * 10 + selfcard.value;
        }
        let mut otherres = 0;
        for othercard in other.cards.iter() {
            otherres = otherres * 10 + othercard.value;
        }

        selfres.cmp(&otherres)
    }
}

impl Play {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.cards.sort_by(|a, b| b.cmp(a));
    }
}
