use std::{io::{BufReader, stdin}, collections::HashSet};

use day14::read_problem;

fn main() {
    let mut spins = 0usize;
    let mut tmap = read_problem(BufReader::new(stdin()));

    let mut set = HashSet::new();
    let mut hash_series = vec![];
    set.insert(tmap.hash());
    hash_series.push(tmap.hash());

    loop {
        spins += 1;
        tmap = tmap.spin_cycle();
        let hash = tmap.hash();
        if set.contains(&hash) {
            for (k, &h) in hash_series.iter().enumerate() {
                if h == hash {
                    let cycle_length = spins - k;
                    let left = (1000000000 - k) % cycle_length;
                    println!("Cycle length: {cycle_length}");
                    println!("Left: {}", left);

                    for _ in 0..left {
                        tmap = tmap.spin_cycle();
                    }
                }
            }
            break;
        } else {
            set.insert(hash);
            hash_series.push(hash);
        }
    }


    println!("Expected load: {}", tmap.load_on_beams());
}
