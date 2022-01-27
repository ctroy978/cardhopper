use cardlib::*;

fn main() {
    let mut player = Hand::new();
    let cards: Vec<usize> = vec![6, 20, 8, 22, 10];
    for c in cards.iter() {
        player.deal_card(Card::new(*c));
    }
    println!("{}", player);
    let x = player.find_matches();
    let y = player.find_strait();
    println!("num of pairs: {:?}, is_strait: {}", x, y);
}
