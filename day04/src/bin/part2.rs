use std::io::stdin;
use day04::Card;

fn main() {
    let cards = stdin()
        .lines()
        .map(|st| Card::parse(&st.unwrap()));

    let mut copies = vec![1usize];

    for (idx, card) in cards.enumerate() {
        if copies.len() == idx {
            copies.push(1);
        }
        let multiplier = copies[idx];
        let winners = card.winner_count();
        let left_to_count = copies.len() - (idx + 1);
        let missing = winners.saturating_sub(left_to_count);
        if missing > 0 {
            let mut new = vec![1usize; missing];
            copies.append(&mut new);
        }
        copies[idx+1..=idx+winners]
            .iter_mut()
            .for_each(|elem| *elem += multiplier);
    }

    println!("The total scratchcards: {}", copies.iter().sum::<usize>());
}
