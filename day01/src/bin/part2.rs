use std::io::stdin;

const NUMBER_OPTIONS: &[(&str, char, usize)] = &[
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

fn parse_string(st: &str) -> Option<(char, usize)> {
    for (mtch, value, size) in NUMBER_OPTIONS {
        if st.starts_with(mtch) {
            return Some((*value, *size))
        }
    }

    None
}

fn collect_values(st: &str) -> u32 {
    let mut idx = 0;
    let mut digits = vec![];

    while idx < st.len() {
        idx += match parse_string(&st[idx..]){
            Some((value, size)) => {
                digits.push(value);
                std::cmp::max(1, size - 1)
            }
            _ => 1
        };
    }

    let result: String = vec![digits.first().unwrap(), digits.last().unwrap()].into_iter().collect();

    result.parse::<u32>().unwrap()
}

fn main() {
    let lines = stdin().lines().collect::<Vec<_>>();

    let values = lines
        .iter()
        .map(|line| collect_values(line.as_ref().unwrap()))
        .sum::<u32>();

    println!("Sum of all the calibration numbers (spelling edition): {}", values);
}

#[cfg(test)]
mod tests {
    use crate::collect_values;

    #[test]
    fn pass_previous_tests() {
        assert_eq!(collect_values("1abc2"), 12);
        assert_eq!(collect_values("pqr3stu8vwx"), 38);
        assert_eq!(collect_values("a1b2c3d4e5f"), 15);
        assert_eq!(collect_values("treb7uchet"), 77);
    }

    #[test]
    fn find_non_overlapping_numbers() {
        assert_eq!(collect_values("eightwothree"), 83);
        assert_eq!(collect_values("onefoursixnine"), 19);
        assert_eq!(collect_values("five"), 55);
    }

    #[test]
    fn mixed_digits_and_text() {
        assert_eq!(collect_values("abcone2threexyz"), 13);
        assert_eq!(collect_values("7pqrstsixteen"), 76);
    }

    #[test]
    fn overlapping_numbers() {
        assert_eq!(collect_values("zoneight234"), 14);
        assert_eq!(collect_values("abtwonefoo"), 21);
    }
}
