use std::io::stdin;

use day04::Card;

fn main() {
    let cards = stdin()
        .lines()
        .map(|st| Card::parse(&st.unwrap()).value());

    println!("The total value of the cards is {}", cards.sum::<u64>());
}
