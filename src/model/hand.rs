use super::{card::Card, play::Play};

const MAX_SIZE: usize = 9;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
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

    pub fn play_cards(&mut self, chosen_cards: Vec<usize>) -> Result<Play, ()> {
        if self.is_playable(chosen_cards.clone()).is_err() {
            return Err(());
        }

        let mut card: Card;
        let mut res = Play::default();

        for id in chosen_cards.clone().iter() {
            card = self.cards.remove(*id);
            res.add_card(card);
        }

        Ok(res)
    }

    pub fn is_playable(&mut self, chosen_cards: Vec<usize>) -> Result<(), ()> {
        if chosen_cards.len() < 1 {
            return Err(());
        }

        let card_value = (self.cards[chosen_cards[0]]).value;
        let card_color = (self.cards[chosen_cards[0]]).card_color;

        let mut order_on_values = true;
        let mut order_on_colors = true;

        let mut iterate_object: Card;

        for element in chosen_cards.iter() {
            iterate_object = self.cards[*element];
            if iterate_object.value != card_value {
                order_on_values = false;
            }
            if iterate_object.card_color != card_color {
                order_on_colors = false;
            }
        }

        if !order_on_colors && !order_on_values {
            return Err(());
        }
        Ok(())
    }
}
