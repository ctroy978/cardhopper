mod games;

use crate::games::stud::*;
use cardlib::*;

use combinations::Combinations;

fn main() {
    let mut players = vec!["tim", "lin", "corn"];
    //play_stud(players);

    let mut player = Hand::new("tester");
    let cards: Vec<usize> = vec![26, 28, 33, 34, 12, 13, 35];
    for c in cards.iter() {
        player.deal_card(Card::new(*c));
    }

    //begin combination
    //create a vec of the values of all cards in hand
    let mut vec_values = Vec::new();
    for c in player.hand.iter() {
        vec_values.push(c.get_value());
    }
    let computed: Vec<_> = Combinations::new(vec_values, 5).collect();
    let mut best = PokerHand::HighCard;
    let mut best_hand = Hand::new("tester");
    for (i, hand) in computed.iter().enumerate() {
        best = PokerHand::HighCard;
        let mut test_hand = Hand::new("tester_spin");
        for c in hand.iter() {
            test_hand.deal_card(Card::new(*c));
        }
        let ph = test_hand.find_poker_hand().unwrap();
        println!("{} {:?}", i, ph);
        if ph > best {
            best = ph;
            best_hand = test_hand;
        }
    }
    println!("{:?}", best);
    println!("{}", best_hand)
}
