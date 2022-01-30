use cardlib::*;
///takes a vector of strings representing names
/// of players.
pub fn play_hicard(player_names: Vec<&str>) -> String {
    let mut deck = Deck::new();
    let mut players: Vec<Hand> = Vec::new();
    for player in player_names.iter() {
        players.push(Hand::new(player));
    }
    for player in players.iter_mut() {
        match deck.get_top_card() {
            Some(card) => player.deal_card(card),
            None => println!("no card"),
        }
    }
    //determins winner
    let mut winner = "nobody".to_string();
    let mut top_card = 0;
    for player in players.iter() {
        println!("{}", player);
        let score = player.hi_card();
        if score > top_card {
            top_card = score;
            winner = player.get_name();
        }
    }
    winner
}
