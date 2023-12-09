pub fn find_next(series: &Vec<i64>) -> i64 {
    let length = series.len();
    let next_row: Vec<i64> = series[..length-1].iter().zip(series[1..].iter())
        .map(|(&a, &b)| b - a)
        .collect();

    let last = *series.last().unwrap();
    if !next_row.iter().all(|&k| k == 0) {
        last + find_next(&next_row)
    } else {
        last
    }
}

pub fn find_prev(series: &Vec<i64>) -> i64 {
    let length = series.len();
    let next_row: Vec<i64> = series[..length-1].iter().zip(series[1..].iter())
        .map(|(&a, &b)| b - a)
        .collect();

    let first = *series.first().unwrap();
    if !next_row.iter().all(|&k| k == 0) {
        first - find_prev(&next_row)
    } else {
        first
    }
}
