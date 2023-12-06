use std::io::stdin;

use day06::parse_kerning;

fn main() {
    let mut input = stdin().lines();
    let time_line = input.next().unwrap().unwrap();
    let distance_line = input.next().unwrap().unwrap();
    let race = parse_kerning(&time_line, &distance_line);

    let (min, max) = race.solve();

    eprintln!("Solutions: {:#?}", max - min + 1);
}
