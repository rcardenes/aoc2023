use std::io::stdin;

use day07::Hand;

fn main() {
    let mut hands: Vec<Hand> = stdin().lines()
        .map(|line| line.unwrap().as_str().into())
        .collect();

    hands.sort();

    let winnings = hands.into_iter()
        .enumerate()
        .fold(0, |acc, (n, hand)| {
            acc + (hand.bid * ((n as u64) + 1))
        });

    print!("Total winnings: {winnings}");
}
