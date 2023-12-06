use std::io::stdin;

use day06::parse_problem;

fn main() {
    let mut input = stdin().lines();
    let time_line = input.next().unwrap().unwrap();
    let distance_line = input.next().unwrap().unwrap();
    let races = parse_problem(&time_line, &distance_line);

    let n_solutions = races
        .iter()
        .map(|r| {
            let sol = r.solve();
            (sol.1 - sol.0) + 1
        });

    let answer = n_solutions.reduce(|acc, n| acc * n).unwrap();

    eprintln!("Solutions: {:#?}", answer);
}
