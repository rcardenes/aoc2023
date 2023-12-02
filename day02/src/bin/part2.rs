use std::io::stdin;
use day02::parse_line;

fn main() {
    let result = stdin().lines()
        .map(|line| parse_line(line.unwrap().trim()))
        .map(|game| game.minimal_set().power())
        .sum::<usize>();

    eprintln!("The total power from the minimal sets is {}", result);
}
