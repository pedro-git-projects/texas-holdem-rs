use std::collections::VecDeque;

use crate::{
    cards::{cards::Card, deck::Deck},
    player::player::Player,
};

#[derive(Debug, Clone)]
pub struct Game {
    players: VecDeque<Player>,
    dealer_index: usize,
    deck: Deck,
    community_cards: Vec<Card>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let mut deck = Deck::default();
        deck.shuffle();
        Self {
            players: players.into(),
            dealer_index: 0,
            deck,
            community_cards: Vec::new(),
        }
    }

    pub fn rotate_dealer(&mut self) {
        self.dealer_index = (self.dealer_index + 1) % self.players.len();
    }

    pub fn get_dealer(&self) -> &Player {
        &self.players[self.dealer_index]
    }

    pub fn deal_cards(&mut self) {
        for player in self.players.iter_mut() {
            player.clear_cards();
        }

        for _ in 0..2 {
            for player in self.players.iter_mut() {
                player.receive_card(self.deck.deal().unwrap());
            }
        }

        self.deal_community_cards(3); // Flop
        self.deal_community_cards(1); // Turn
        self.deal_community_cards(1); // River
    }

    fn deal_community_cards(&mut self, num_cards: usize) {
        for _ in 0..num_cards {
            let card = self.deck.deal();
            self.community_cards.push(card.unwrap());
        }
    }

    pub fn get_community_cards(&self) -> &[Card] {
        &self.community_cards
    }
}
