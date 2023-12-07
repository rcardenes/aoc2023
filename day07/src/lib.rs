use std::collections::HashMap;

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

impl Kind {
    fn from_cards(cards: &[Card]) -> anyhow::Result<Kind> {
        if cards.len() != 5 {
            bail!("Illegal hand size: {}", cards.len())
        }

        let mut unique = HashMap::new();
        for card in cards {
            let count = unique.entry(card).or_insert(0);
            *count += 1;
        }

        let mut card_count = unique.values().cloned().collect::<Vec<_>>();
        card_count.sort();

        Ok(match unique.len() {
            5 => Kind::High,
            4 => Kind::Pair,
            3 => if card_count == [1, 2, 2] { Kind::TwoPair } else { Kind::ThreeOf }
            2 => if card_count == [1, 4] { Kind::FourOf } else { Kind::Full }
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

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (hand_raw, bid) = value.split_once(' ').unwrap();
        let cards: Vec<Card> = hand_raw.chars().map(|f| Card::from_char(f, false).unwrap()).collect();
        let kind = Kind::from_cards(&cards).unwrap();
        let bid = bid.parse::<u64>().unwrap();

        Hand {
            kind,
            cards,
            bid
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Hand, Kind};

    const SAMPLE_INPUT: &str = include_str!("../input.small");

    #[test]
    fn parse_line () {
        let line = "32T3K 765";
        let hand: Hand = line.into();

        assert_eq!(hand, Hand::new(
                    Kind::Pair,
                    vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                    765));

        let line = "QQQJA 483";
        let hand: Hand = line.into();

        assert_eq!(hand, Hand::new(
                    Kind::ThreeOf,
                    vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                    483));
    }

    #[test]
    fn compare_hands () {
        let hands: Vec<Hand> = SAMPLE_INPUT.lines()
            .map(|line| line.into())
            .collect();

        assert!(hands[1] > hands[0]);
    }
}
