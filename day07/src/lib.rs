use std::{collections::HashMap, fmt::Display};

use anyhow::bail;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    High,
    Pair,
    TwoPair,
    ThreeOf,
    Full,
    FourOf,
    FiveOf,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    Joker,
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
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hand {
    kind: Kind,
    cards: Vec<Card>,
    pub bid: u64,
}

impl Hand {
    pub fn from_str(value: &str, with_joker: bool) -> Hand {
        let (hand_raw, bid) = value.split_once(' ').unwrap();
        let cards: Vec<Card> = hand_raw.chars().map(|f| Card::from_char(f, with_joker).unwrap()).collect();
        let kind = Kind::from_cards(&cards).unwrap();
        let bid = bid.parse::<u64>().unwrap();

        Hand {
            kind,
            cards,
            bid
        }
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        match self {
            Card::Joker => 'J',
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'j',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        }.to_string()
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {:?}",
               self.cards
                   .iter()
                   .map(|c| c.to_string())
                   .collect::<Vec<_>>()
                   .join(""),
               self.kind)
    }
}

impl Kind {
    fn from_cards(cards: &[Card]) -> anyhow::Result<Kind> {
        if cards.len() != 5 {
            bail!("Illegal hand size: {}", cards.len())
        }

        let mut unique = HashMap::new();
        let mut jokers = 0;
        // Don't include the jokers in the mapping
        for card in cards {
            if *card == Card::Joker {
                jokers += 1;
            }

            let count = unique.entry(card).or_insert(0);
            *count += 1;
        }

        let mut card_count = unique.values().cloned().collect::<Vec<_>>();
        card_count.sort();


        Ok(match unique.len() {
            5 => if jokers == 0 { Kind::High } else { Kind::Pair },
            // For the following case, jokers could be 1 or 2. In either
            // case we can call Three of a Kind
            4 => if jokers == 0 { Kind::Pair } else { Kind::ThreeOf }
            3 => if card_count == [1, 2, 2] {
                match jokers {
                    0 => Kind::TwoPair,
                    1 => Kind::Full,
                    // The remaining case: there are 2 jokers
                    _ => Kind::FourOf,
                }
            } else { // Has to be [1, 1, 3]
                match jokers {
                    0 => Kind::ThreeOf,
                    // The remaining case: there are either 1 or 3 jokers,
                    // the result is the same
                    _ => Kind::FourOf
                }
            }
            2 => if jokers == 0 {
                if card_count == [1, 4] { Kind::FourOf } else { Kind::Full }
            } else {
                Kind::FiveOf
            }
            1 => Kind::FiveOf,
            _ => unimplemented!() // Can't happen
        })
    }
}

impl Card {
    fn from_char(value: char, with_joker: bool) -> anyhow::Result<Self> {
        Ok(match value {
            'J' => if with_joker { Card::Joker } else { Card::Jack },
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => bail!("Not a valid card face {value:?}")
        })
    }
}

impl Hand {
    #[allow(dead_code)]
    fn new(kind: Kind, cards: Vec<Card>, bid: u64) -> Self {
        Hand { kind, cards, bid }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Hand, Kind};

    const SAMPLE_INPUT: &str = include_str!("../input.small");

    #[test]
    fn parse_line () {
        let line = "32T3K 765";
        let hand = Hand::from_str(line, false);

        assert_eq!(hand, Hand::new(
                    Kind::Pair,
                    vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                    765));

        let line = "QQQJA 483";
        let hand = Hand::from_str(line, false);

        assert_eq!(hand, Hand::new(
                    Kind::ThreeOf,
                    vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                    483));
    }

    #[test]
    fn compare_hands () {
        let hands: Vec<Hand> = SAMPLE_INPUT.lines()
            .map(|line| Hand::from_str(line, false))
            .collect();

        assert!(hands[1] > hands[0]);
    }
}
