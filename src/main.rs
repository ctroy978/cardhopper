use cardlib::*;

fn main() {
    let mut player = Hand::new();
    let cards: Vec<usize> = vec![6, 19, 33, 46, 12];
    for c in cards.iter() {
        player.deal_card(Card::new(*c));
    }
    player.sort_hand();
    println!("{}", player);
    let x = player.find_matches();
    println!("num of pairs: {:?}", x);
}
