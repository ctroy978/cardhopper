use std::collections::HashMap;
use std::fmt;

const FULLDECK: usize = 52;
const DIVIDER: usize = 13; //number for math tricks to assign suit and rank

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    heart,
    diamond,
    club,
    spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    ten,
    jack,
    queen,
    king,
    ace,
    joker,
}

///card can have a value between 0-51.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    value: usize,
}

pub struct Deck {
    deck: HashMap<usize, Card>,
}

impl Card {
    ///value must be between 0-51
    pub fn new(value: usize) -> Self {
        Self { value }
    }
    pub fn get_suit(&self) -> Suit {
        let suits = [Suit::club, Suit::diamond, Suit::heart, Suit::spade];
        let pick = self.value / DIVIDER;
        suits[pick]
    }
    pub fn get_rank(&self) -> Rank {
        let rank = match self.value % DIVIDER {
            0 => Rank::two,
            1 => Rank::three,
            2 => Rank::four,
            3 => Rank::five,
            4 => Rank::six,
            5 => Rank::seven,
            6 => Rank::eight,
            7 => Rank::nine,
            8 => Rank::ten,
            9 => Rank::jack,
            10 => Rank::queen,
            11 => Rank::king,
            12 => Rank::ace,
            _ => Rank::joker,
        };
        rank
    }
    ///get rank as number to compare greater than or less than
    pub fn get_rank_usize(&self) -> usize {
        (self.value % 13) + 2
    }
    ///get suit as number to compare greater than or less than
    /// will come out alphabetical club-0, diamond-1, heart-2, spade-3
    pub fn get_suit_usize(&self) -> usize {
        self.value / 13
    }
}
impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "-{:?} of {:?}s",
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
