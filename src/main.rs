use game::game::Game;
use player::player::Player;

mod cards;
mod game;
mod player;

fn main() {
    let player1 = Player::new(350);
    let player2 = Player::new(350);
    let players = vec![player1, player2];
    let mut game = Game::new(players);

    game.deal_cards();

    let dealer = game.get_dealer();
    println!("{:?}", dealer);

    let community_cards = game.get_community_cards();
    println!("{:?}", community_cards);
}
