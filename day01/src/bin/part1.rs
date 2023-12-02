use std::io::stdin;

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

    println!("Sum of all the calibration numbers: {}", values);
}

#[cfg(test)]
mod tests {
    use crate::collect_values;

    #[test]
    fn find_numbers_at_ends() {
        assert_eq!(collect_values("1abc2"), 12);
    }

    #[test]
    fn find_numbers_inside_the_string() {
        assert_eq!(collect_values("pqr3stu8vwx"), 38);
    }

    #[test]
    fn find_only_first_and_last_numbers() {
        assert_eq!(collect_values("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn only_one_number_in_string() {
        assert_eq!(collect_values("treb7uchet"), 77);
    }
}
