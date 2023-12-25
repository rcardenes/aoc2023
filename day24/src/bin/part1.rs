use std::io::{BufReader, stdin};
use day24::*;

static BOUNDARIES: (i64, i64) = (200000000000000, 400000000000000);

fn main() {
    let stones = read_problem(BufReader::new(stdin()));
    let mut intersections = 0;

    for (k, st1) in stones[..stones.len()-1].iter().enumerate() {
        for st2 in stones[k+1..].iter() {
            if let Some(inter) = st1.intersection(st2) {
                if st1.is_future(&inter) && st2.is_future(&inter) && inter.within_2d_boundaries(BOUNDARIES.0, BOUNDARIES.1) {
                    intersections += 1;
                    eprintln!("{st1} || {st2}");
                }
            }
        }
    }

    println!("Total intersections within boundaries: {intersections}");
}
