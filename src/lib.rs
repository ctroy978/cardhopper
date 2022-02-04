use std::collections::HashMap;
use std::fmt;

use thiserror::Error;

const FULLDECK: usize = 52;
const DIVIDER: usize = 13; //number for math tricks to assign suit and rank

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CardError {
    #[error("Card not available")]
    CardNotFound,
    #[error("Out of bounds. No card at location")]
    OutOfBounds,
    #[error("Not a five card poker hand")]
    NotFullHand,
    #[error("Not a recognized id")]
    NotCardId,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
    RoyalFlush,
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
    name: String,
    pub hand: Vec<Card>,
}

impl Card {
    /// value must be between 0-51
    /// always assums a 52 deck of cards.
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
        (self.value % 13) + 2 //two is added because cards start at 2 and computers start at 0.
    }
    ///get suit as usize to compare greater than or less than
    /// will come out alphabetical club-0, diamond-1, heart-2, spade-3
    pub fn get_suit_usize(&self) -> usize {
        self.value / 13
    }
    ///get value from card
    pub fn get_value(&self) -> usize {
        self.value
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
    ///build a new deck of 52 cards
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
    pub fn new(name: &str) -> Self {
        let hand = Vec::new();
        let name = name.to_owned();
        Self { name, hand }
    }
    ///return name of hand
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    ///deal one card to player hand
    pub fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }
    ///sort cards in hand
    pub fn sort_by_suit(&mut self) {
        self.hand.sort();
    }

    ///remove a card from hand
    pub fn discard(&mut self, id: &str) -> Result<Card, CardError> {
        let mut id_value: usize = 0;
        match value_from_id(id) {
            Ok(value) => id_value = value,
            Err(e) => return Err(e),
        }
        for (i, card) in self.hand.iter_mut().enumerate() {
            if card.get_value() == id_value {
                return Ok(self.hand.remove(i));
            }
        }
        Err(CardError::CardNotFound)
    }

    ///return usize value of high card in hand
    pub fn hi_card(&self) -> usize {
        let mut hi_card: usize = 0;
        for card in self.hand.iter() {
            if card.get_rank_usize() > hi_card {
                hi_card = card.get_rank_usize();
            }
        }
        hi_card
    }

    ///returns number of pairs in a hand by rank.
    /// first two spots in array are 0 and 1 and
    /// won't get filled. filled array is 2 - ace(15);
    fn find_matches(&self) -> Vec<usize> {
        let mut matches = vec![0; 15];
        for card in self.hand.iter() {
            matches[card.get_rank_usize()] += 1;
        }
        matches
    }

    ///returns true if hand has a strait.
    fn is_straight(&self) -> bool {
        let mut strait_vec: Vec<usize> = Vec::new();
        for card in self.hand.iter() {
            strait_vec.push(card.get_rank_usize());
        }
        strait_vec.sort();
        let mut base = strait_vec[0];
        for i in strait_vec.iter() {
            if i != &base {
                return false;
            }
            base += 1;
        }
        true
    }
    ///returns true if hand is flush
    fn is_flush(&self) -> bool {
        let base = self.hand[0].get_suit();
        for card in self.hand.iter() {
            if card.get_suit() != base {
                return false;
            }
        }
        true
    }

    ///returns true if hand is strait flush
    fn is_straight_flush(&self) -> bool {
        if self.is_flush() && self.is_straight() {
            return true;
        }
        false
    }

    ///returns true if hand is royal flush
    fn is_royal_flush(&mut self) -> bool {
        self.sort_by_suit();
        if self.is_straight_flush() && self.hand[0].get_rank() == Rank::Ten {
            return true;
        }
        false
    }

    ///returns true if hand has a match equal to
    /// usize in argument. so if num is 3 and hand has
    /// three of a kind, return true. Will not detect two pair.
    /// use is_two_pair for that.
    fn is_match_of_kind(&self, num: usize) -> bool {
        let matches = self.find_matches();
        for cards in matches.iter() {
            if cards == &num {
                return true;
            }
        }
        false
    }

    ///returns true if hand has two pair
    fn is_two_pair(&self) -> bool {
        let matches = self.find_matches();
        let mut pair = 0;
        for a_match in matches {
            if a_match == 2 {
                pair += 1;
            }
        }
        if pair == 2 {
            return true;
        }
        false
    }

    ///returns true if hand is full house
    fn is_full_house(&self) -> bool {
        if self.is_match_of_kind(3) && self.is_match_of_kind(2) {
            return true;
        }
        false
    }

    ///returns best five card hand
    /// must have only five cards
    pub fn find_poker_hand(&mut self) -> Result<PokerHand, CardError> {
        if self.hand.len() != 5 {
            return Err(CardError::NotFullHand);
        }
        if self.is_match_of_kind(4) {
            return Ok(PokerHand::FourKind);
        } else if self.is_full_house() {
            return Ok(PokerHand::FullHouse);
        } else if self.is_match_of_kind(3) {
            return Ok(PokerHand::ThreeKind);
        } else if self.is_two_pair() {
            return Ok(PokerHand::TwoPair);
        } else if self.is_match_of_kind(2) {
            return Ok(PokerHand::OnePair);
        } else if self.is_royal_flush() {
            return Ok(PokerHand::RoyalFlush);
        } else if self.is_straight_flush() {
            return Ok(PokerHand::StraightFlush);
        } else if self.is_flush() {
            return Ok(PokerHand::Flush);
        } else if self.is_straight() {
            return Ok(PokerHand::Straight);
        }
        Ok(PokerHand::HighCard)
    }
}

impl fmt::Display for Hand {
    ///printable hand
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut hand_to_string: String = String::new();
        hand_to_string.push_str(format!("{}\n", self.name).as_str());
        for card in self.hand.iter() {
            hand_to_string.push_str(format!("-{}\n", card).as_str());
        }
        formatter.write_fmt(format_args!("{}", hand_to_string))
    }
}

///takes a two letter str representing the rank:suit of card
/// and returns card value. So 2c => 0  = two of clubs.
pub fn value_from_id(id: &str) -> Result<usize, CardError> {
    let id_val = match id {
        "2c" => 0,
        "3c" => 1,
        "4c" => 2,
        "5c" => 3,
        "6c" => 4,
        "7c" => 5,
        "8c" => 6,
        "9c" => 7,
        "10c" => 8,
        "jc" => 9,
        "qc" => 10,
        "kc" => 11,
        "ac" => 12,
        "2d" => 13,
        "3d" => 14,
        "4d" => 15,
        "5d" => 16,
        "6d" => 17,
        "7d" => 18,
        "8d" => 19,
        "9d" => 20,
        "10d" => 21,
        "jd" => 22,
        "qd" => 23,
        "kd" => 24,
        "ad" => 25,
        "2h" => 26,
        "3h" => 27,
        "4h" => 28,
        "5h" => 29,
        "6h" => 30,
        "7h" => 31,
        "8h" => 32,
        "9h" => 33,
        "10h" => 34,
        "jh" => 35,
        "qh" => 36,
        "kh" => 37,
        "ah" => 38,
        "2s" => 39,
        "3s" => 40,
        "4s" => 41,
        "5s" => 42,
        "6s" => 43,
        "7s" => 44,
        "8s" => 45,
        "9s" => 46,
        "10s" => 47,
        "js" => 48,
        "qs" => 49,
        "ks" => 50,
        "as" => 51,
        _ => return Err(CardError::NotCardId),
    };
    Ok(id_val)
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
        let mut player = Hand::new("tester");
        let cards: Vec<usize> = vec![5, 22, 4, 33, 48];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        player.sort_by_suit();
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
    #[test]
    fn test_matching() {
        let mut player = Hand::new("tester");
        let cards: Vec<usize> = vec![6, 19, 33, 46, 12];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        assert_eq!(
            player.find_matches(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_royal_chk() {
        let mut player = Hand::new("tester");
        let cards: Vec<usize> = vec![8, 9, 10, 11, 12];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        assert_eq!(player.is_royal_flush(), true);
    }

    #[test]
    fn test_full_house_chk() {
        let mut player = Hand::new("tester");
        let cards: Vec<usize> = vec![6, 19, 32, 1, 14];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        assert_eq!(player.is_full_house(), true);
    }

    #[test]
    fn test_stait_chk() {
        let mut player = Hand::new("tester");
        let cards: Vec<usize> = vec![6, 20, 8, 22, 10];
        for c in cards.iter() {
            player.deal_card(Card::new(*c));
        }
        assert_eq!(player.is_straight(), true);
    }
}
