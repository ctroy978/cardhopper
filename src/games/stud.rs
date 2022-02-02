use super::user_input::*;
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
    //begin play
    loop {
        for player in players.iter_mut() {
            println!("{}", &player);
            println!("{:?}", &player.find_poker_hand());
            discard_round(player, &mut deck);
        }
    }
}

fn discard_round(player: &mut Hand, deck: &mut Deck) {
    loop {
        //get user input on which cards to discard
        let discard_ids = user_input_string(
        "List all cards to discard. Seperate cards with spaces. \n ex: kh 2d = king of hearts and two of dimonds",
    )
    .unwrap();
        let choices: Vec<&str> = discard_ids.split(" ").collect();

        //check to make sure user is ok with choices
        if !verify_choices(&choices) {
            continue;
        }

        for id in choices.iter() {
            match player.discard(id.trim()) {
                Ok(_) => match deck.get_top_card() {
                    Some(card) => player.deal_card(card),
                    None => println!("no card available"),
                },
                Err(e) => println!("{}", e),
            }
        }
        println!("{}", &player);
        println!("{:?}", player.find_poker_hand());
        user_input_string("hit enter when finished looking at your hand.").unwrap();
        break;
    }
}

fn verify_choices(choices: &Vec<&str>) -> bool {
    for id in choices.iter() {
        match value_from_id(id) {
            Ok(val) => {
                let temp = Card::new(val);
                println!("{}", temp);
            }
            Err(e) => println!("{}", e),
        };
    }
    match user_input_string("Are you OK with this discard?")
        .unwrap()
        .as_str()
    {
        "yes" | "y" => return true,
        _ => return false,
    }
}
