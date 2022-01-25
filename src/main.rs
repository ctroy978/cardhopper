use cardlib::*;

fn main() {
    let mut deck = Deck::new();
    let s = deck.get_top_card().unwrap();
    let t = deck.get_top_card().unwrap();
    println!("{}", s);
    println!("{}", t);
    let card1 = Card::new(4);
    let card2 = Card::new(17);

    let b = (card1.get_suit() == card2.get_suit());
    let r = (card1.get_rank() == card2.get_rank());
    println!("{} {}", card1, card2);
    println!("{}", card1.get_rank_usize());
    println!("{}", card1.get_suit_usize());
    println!("{}", r);
}
