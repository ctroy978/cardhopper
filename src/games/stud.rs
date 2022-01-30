use cardlib::*;

pub fn play_stud(player_names: Vec<&str>) {
    let mut deck = Deck::new();
    let mut players: Vec<Hand> = Vec::new();
    //give each player a hand
    for player in player_names.iter() {
        players.push(Hand::new(player));
    }
    //deal five cards
    for player in players.iter_mut() {
        for _ in 0..5 {
            match deck.get_top_card() {
                Some(card) => player.deal_card(card),
                None => println!("no card"),
            }
        }
    }
    for player in players.iter() {
        println!("{}", player);
    }
}
