use std::io::stdin;
use day02::{GameData, parse_line};

fn main() {
    let ref_data = GameData::new (12, 13, 14);
    let result = stdin().lines()
        .map(|line| parse_line(line.unwrap().trim()))
        .filter(|game| game.is_possible(&ref_data))
        .map(|game| game.id)
        .sum::<usize>();
    println!("The sum of the possible game IDs is: {result}");
}
