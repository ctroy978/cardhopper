use cardlib::*;

fn main() {
    let mut player = Hand::new();
    let cards: Vec<usize> = vec![6, 19, 2, 7, 20];
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
    let h = player.hi_card();
    println!("hi: {}", h);
    let three = player.is_match_of_kind(3);
    println!("three: {}", three);
    let full = player.is_full_house();
    println!("full: {}", full);
    let twopair = player.is_two_pair();
    println!("two pair: {}", twopair);
}
