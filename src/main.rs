use cardlib::*;

fn main() {
    let mut player = Hand::new();
    let cards: Vec<usize> = vec![8, 9, 10, 11, 12];
    for c in cards.iter() {
        player.deal_card(Card::new(*c));
    }
    println!("{}", player);
    let x = player.find_matches();
    let y = player.is_strait();
    let z = player.is_flush();
    println!("num of pairs: {:?}, is_strait: {}, is flush: {}", x, y, z);
    let xyz = player.is_royal_flush();
    println!("{}", xyz);
}
