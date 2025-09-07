use super::card::Card;

const MAX_SIZE: usize = 9;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand
{
    pub fn add_card(&mut self, card: Card) -> Result<usize, ()> {
        if self.cards.len() >= MAX_SIZE {
            return Err(());
        } 
        self.cards.push(card);
        Ok(self.cards.len())
    }

    pub fn order_on_color(&mut self) {
        use std::collections::HashMap;

        let mut color_order = HashMap::new();
        let mut next_index = 0;

        for card in &self.cards {
            color_order.entry(card.card_color).or_insert_with(|| {
                let idx = next_index;
                next_index += 1;
                idx
            });
        }

        self.cards.sort_by(|a, b| {
            let a_idx = color_order[&a.card_color];
            let b_idx = color_order[&b.card_color];
            let color_cmp = a_idx.cmp(&b_idx);
            if color_cmp == std::cmp::Ordering::Equal {
                a.value.cmp(&b.value)
            } else {
                color_cmp
            }
        });
    }
}
