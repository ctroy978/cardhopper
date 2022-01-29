mod games;

use crate::games::hicard::*;
use cardlib::*;

fn main() {
    play_hicard(3);

    let v = vec![3, 4, 5, 6];
    let mut player = Hand::new();
    for c in v.iter() {
        player.deal_card(Card::new(*c));
    }
    println!("{}", player);
    let z = player.discard(7);
    player.discard(2);
    player.discard(1);
    println!("{}", player);
    match z {
        Ok(card) => println!("{}", card),
        Err(e) => println!("{}", e),
    }
}
