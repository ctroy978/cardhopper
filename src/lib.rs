use std::collections::HashMap;
use std::fmt;

const FULLDECK: usize = 52;
const DIVIDER: usize = 13; //number for math tricks to assign suit and rank

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

///card can have a value between 0-51.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    value: usize,
}

pub struct Deck {
    deck: HashMap<usize, Card>,
}

pub struct Hand {
    hand: Vec<Card>,
}

impl Card {
    ///value must be between 0-51
    pub fn new(value: usize) -> Self {
        Self { value }
    }
    pub fn get_suit(&self) -> Suit {
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
        let pick = self.value / DIVIDER;
        suits[pick]
    }
    pub fn get_rank(&self) -> Rank {
        let rank = match self.value % DIVIDER {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => Rank::Joker,
        };
        rank
    }
    ///get rank as usize to compare greater than or less than
    pub fn get_rank_usize(&self) -> usize {
        (self.value % 13) + 2
    }
    ///get suit as usize to compare greater than or less than
    /// will come out alphabetical club-0, diamond-1, heart-2, spade-3
    pub fn get_suit_usize(&self) -> usize {
        self.value / 13
    }
}

impl fmt::Display for Card {
    ///printable card
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{:?} of {:?}s",
            self.get_rank(),
            self.get_suit()
        ))
    }
}
impl Deck {
    pub fn new() -> Self {
        let mut deck = HashMap::new();
        for i in 0..FULLDECK {
            deck.insert(i, Card::new(i));
        }
        Self { deck }
    }
    pub fn get_card(&mut self, value: usize) -> Option<Card> {
        match self.deck.remove(&value) {
            Some(card) => return Some(card),
            None => return None,
        }
    }
    pub fn get_top_card(&mut self) -> Option<Card> {
        match self.get_top_card_key() {
            Some(key) => return self.get_card(key),
            None => return None,
        }
    }
    fn get_top_card_key(&mut self) -> Option<usize> {
        let mut all_keys = self.deck.keys();
        let card_key = all_keys.next();
        match card_key {
            Some(key) => return Some(*key),
            None => return None,
        }
    }
}

impl Hand {
    ///create a new hand for a player
    pub fn new() -> Self {
        let hand = Vec::new();
        Self { hand }
    }
    ///deal one card to player hand
    pub fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }
    ///sort cards in hand
    pub fn sort_hand(&mut self) {
        self.hand.sort();
    }
}
impl fmt::Display for Hand {
    ///printable hand
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut hand_to_string: String = String::new();
        for card in self.hand.iter() {
            hand_to_string.push_str(format!("{}\n", card).as_str());
        }
        formatter.write_fmt(format_args!("{}", hand_to_string))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_card_suit() {
        let card1 = Card::new(5);
        assert_eq!(Suit::Club, card1.get_suit());
    }
    #[test]
    fn card_suit_eq() {
        let card1 = Card::new(5);
        let card2 = Card::new(4);
        assert_eq!(card2.get_suit(), card1.get_suit());
    }
    #[test]
    fn correct_card_rank() {
        let card1 = Card::new(5);
        assert_eq!(Rank::Seven, card1.get_rank());
    }
    #[test]
    fn hand_sorted() {
        let mut player = Hand::new();
        let cards: Vec<usize> = vec![5, 22, 4, 33, 48];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        player.sort_hand();
        assert_eq!(
            player.hand,
            vec![
                Card::new(4),
                Card::new(5),
                Card::new(22),
                Card::new(33),
                Card::new(48),
            ]
        );
    }
}
