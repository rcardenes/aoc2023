use std::io::stdin;
use anyhow::{Result, bail};

fn char_to_int(c: char) -> Result<u32> {
    Ok(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => bail!("Not a valid digit")
    })
}


fn parse_string(st: &str) -> Option<(char, usize)> {
    let number_options = vec![
        ("zero", '0', 4),
        ("one", '1', 3),
        ("two", '2', 3),
        ("three", '3', 5),
        ("four", '4', 4),
        ("five", '5', 4),
        ("six", '6', 3),
        ("seven", '7', 5),
        ("eight", '8', 5),
        ("nine", '9', 4),
        ("0", '0', 1),
        ("1", '1', 1),
        ("2", '2', 1),
        ("3", '3', 1),
        ("4", '4', 1),
        ("5", '5', 1),
        ("6", '6', 1),
        ("7", '7', 1),
        ("8", '8', 1),
        ("9", '9', 1),
        ];
    for (mtch, value, size) in &number_options {
        if st.starts_with(mtch) {
            return Some((*value, *size))
        }
    }

    None
}

fn collect_values2(st: &str) -> u32 {
    let mut idx = 0;
    let mut digits = vec![];

    for idx in 0..st.len() {
        if let Some((value, size)) = parse_string(&st[idx..]) {
            digits.push(value);
        }
    }

    let result: String = vec![digits.first().unwrap(), digits.last().unwrap()].into_iter().collect();

    result.parse::<u32>().unwrap()
}

fn collect_values(st: &str) -> u32 {
    let digits = '0'..='9';
    let d_list = st.chars()
        .filter(|c| digits.contains(c))
        .collect::<Vec<_>>();

    let result: String = vec![d_list.first().unwrap(), d_list.last().unwrap()].into_iter().collect();

    result.parse::<u32>().unwrap()
}

fn main() {
    let lines = stdin().lines().collect::<Vec<_>>();

    let values = lines
        .iter()
        .map(|line| collect_values(line.as_ref().unwrap()))
        .sum::<u32>();

    println!("First: {}", values);

    let values = lines
        .iter()
        .map(|line| collect_values2(line.as_ref().unwrap()))
        .sum::<u32>();

    println!("Second: {}", values);
}
