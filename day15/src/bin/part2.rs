use std::io::{BufReader, BufRead, stdin};

use day15::{LensBox, hash_algo};

fn main() {
    let mut stream = BufReader::new(stdin());
    let mut buf = String::from("");
    stream.read_line(&mut buf).unwrap();

    let mut boxes = vec![LensBox::new();256];

    for label in buf.trim().split(",") {
        if label.ends_with("-") {
            let s = &label[..label.len()-1];
            let index = hash_algo(s) as usize;
            boxes[index] = boxes[index].remove(s);
        } else {
            let (s, v) = label.split_once('=').unwrap();
            let index = hash_algo(s) as usize;
            boxes[index] = boxes[index].replace(s, v.parse::<u8>().unwrap());
        }
    }

    let mut power = 0;

    for (k, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            eprintln!("Box {k:03}: {b:#?}");
        }
        power += b.power() * (k as u32 + 1);
    }

    println!("Total power: {power}");
}
