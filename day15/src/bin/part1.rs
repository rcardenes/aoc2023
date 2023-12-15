use std::io::{BufReader, BufRead, stdin};

use day15::hash_algo;

fn main() {
    let mut stream = BufReader::new(stdin());
    let mut buf = String::from("");
    stream.read_line(&mut buf).unwrap();

    let res = buf.trim().split(",").fold(0u32, |acc, s| acc + hash_algo(s) as u32);
    println!("{res}");
}
