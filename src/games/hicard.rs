use cardlib::*;

pub fn play_hicard(num_player: usize) {
    let mut deck = Deck::new();
    let mut players: Vec<Hand> = Vec::new();
    for _ in 0..num_player {
        players.push(Hand::new());
    }
    for player in players.iter_mut() {
        match deck.get_top_card() {
            Some(card) => player.deal_card(card),
            None => println!("no card"),
        }
    }

    for player in players.iter() {
        println!("{}", player);
    }
}
