use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Card {
    id: usize,
    winning: HashSet<u32>,
    owned: Vec<u32>,
}

impl Card {
    pub fn parse(st: &str) -> Self {
        let (header, numbers) = st.split_once(": ").unwrap();
        let id_raw = header.split_whitespace().last().unwrap();
        let (winning_raw, owned_raw) = numbers.split_once(" | ").unwrap();
        let winning = winning_raw.split_whitespace()
            .map(|num| num.parse::<u32>().unwrap());
        let owned = owned_raw.split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();


        Card {
            id: id_raw.parse::<usize>().unwrap(),
            winning: HashSet::from_iter(winning),
            owned
        }
    }

    pub fn winner_count(&self) -> usize {
        self.owned
            .iter()
            .filter(|&n| self.winning.contains(n))
            .count()
    }

    pub fn value(&self) -> u64 {
        let matches = self.winner_count();
        if matches > 0 {
            1u64 << (matches - 1)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::Card;

    const SAMPLE_INPUT: &str = include_str!("../input.small");

    #[test]
    fn parse_card() {
        assert_eq!(
            Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                owned: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn find_value() {
        let cards: Vec<u64> = SAMPLE_INPUT
            .lines()
            .map(|st| Card::parse(st).value())
            .collect();

        assert_eq!(cards, [8, 2, 2, 1, 0, 0])
    }
}
