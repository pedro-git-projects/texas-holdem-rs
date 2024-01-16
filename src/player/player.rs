use crate::cards::cards::Card;

#[derive(Debug, Clone)]
pub struct Player {
    cards: Vec<Card>,
    chips: u32,
}

impl Player {
    pub fn new(number_of_chips: u32) -> Self {
        Self {
            cards: Vec::new(),
            chips: number_of_chips,
        }
    }

    pub fn clear_cards(&mut self) {
        self.cards.clear()
    }

    pub fn receive_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
