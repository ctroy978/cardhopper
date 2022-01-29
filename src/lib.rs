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
    #[error("No card at that location")]
    BadLocationRequest,
}

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
    name: String,
    hand: Vec<Card>,
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
    pub fn discard(&mut self, location: usize) -> Result<Card, CardError> {
        if location >= self.hand.len() {
            return Err(CardError::BadLocationRequest);
        }
        Ok(self.hand.remove(location))
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
    pub fn find_matches(&self) -> Vec<usize> {
        let mut matches = vec![0; 15];
        for card in self.hand.iter() {
            matches[card.get_rank_usize()] += 1;
        }
        matches
    }
    ///returns true if hand has a strait.
    pub fn is_strait(&self) -> bool {
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
    pub fn is_flush(&self) -> bool {
        let base = self.hand[0].get_suit();
        for card in self.hand.iter() {
            if card.get_suit() != base {
                return false;
            }
        }
        true
    }
    ///returns true if hand is strait flush
    pub fn is_strait_flush(&self) -> bool {
        if self.is_flush() && self.is_strait() {
            return true;
        }
        false
    }
    ///returns true if hand is royal flush
    pub fn is_royal_flush(&mut self) -> bool {
        self.sort_by_suit();
        if self.is_strait_flush() && self.hand[0].get_rank() == Rank::Ten {
            return true;
        }
        false
    }
    ///returns true if hand has a match equal to
    /// usize in argument. so if num is 3 and hand has
    /// three of a kind, return true. Will not detect two pair.
    /// use is_two_pair for that.
    pub fn is_match_of_kind(&self, num: usize) -> bool {
        let matches = self.find_matches();
        for cards in matches.iter() {
            if cards == &num {
                return true;
            }
        }
        false
    }
    ///returns true if hand has two pair
    pub fn is_two_pair(&self) -> bool {
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
    pub fn is_full_house(&self) -> bool {
        if self.is_match_of_kind(3) && self.is_match_of_kind(2) {
            return true;
        }
        false
    }
}
impl fmt::Display for Hand {
    ///printable hand
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut hand_to_string: String = String::new();
        hand_to_string.push_str(format!("{}\n", self.name).as_str());
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
        assert_eq!(player.is_strait(), true);
    }
}
