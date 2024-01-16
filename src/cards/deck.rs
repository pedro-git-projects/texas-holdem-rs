use rand::{thread_rng, Rng};

use super::cards::{Card, Rank, Suit};

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let cards = Vec::new();
        let mut deck = Deck { cards };

        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                deck.cards.push(Card::new(rank, suit));
            }
        }
        deck
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        for i in (1..self.cards.len()).rev() {
            let j = rng.gen_range(0..i + 1);
            self.cards.swap(i, j);
        }
    }

    pub fn deal(&mut self) -> Option<Card> {
        if self.cards.len() <= 1 {
            self.cards = Default::default();
            self.shuffle();
        }

        self.cards.pop()
    }
}
