#[derive(Debug)]
pub struct RaceInfo {
    duration: u64, // milliseconds
    record: u64,   // millimeters
}

impl RaceInfo {
    fn new(duration: u64, record: u64) -> Self {
        RaceInfo { duration, record }
    }

    // The boat's speed will be "time_pressed mm/ms". Obviously it will be
    // able to travel only for a maximum of the race duration, meaning that
    // pressing the button for the whole duration (or longer) will result in
    // exactly 0mm of travel
    fn distance_travelled(&self, time_pressed: u64) -> u64 {
        let time_travelling = self.duration.saturating_sub(time_pressed);

        time_travelling * time_pressed
    }

    pub fn find_min(&self) -> u64 {
        let mut lim = (0u64, self.duration / 2);
        let mut lowest = 0;

        while lim.0 <= lim.1 {
            let mid = (lim.0 + lim.1) / 2;
            let d = self.distance_travelled(mid);

            if d > self.record {
                lowest = mid;
                lim = (lim.0, mid - 1)
            } else {
                lim = (mid + 1, lim.1)
            }
        }

        lowest
    }

    pub fn find_max(&self) -> u64 {
        let mut highest = self.duration;
        let mut lim = (self.duration / 2, highest);

        while lim.0 <= lim.1 {
            let mid = (lim.0 + lim.1) / 2;
            let d = self.distance_travelled(mid);

            if d <= self.record {
                lim = (lim.0, mid - 1)
            } else {
                highest = mid;
                lim = (mid + 1, lim.1)
            }
        }

        highest
    }

    // Returns two numbers: Minimum and maximum number of seconds the button
    // can be pressed that will let you beat the record
    pub fn solve(&self) -> (u64, u64) {
        (self.find_min(), self.find_max())
    }
}

pub fn parse_problem(time_line: &str, distance_line: &str) -> Vec<RaceInfo> {
    let times = (time_line.split_once(":").unwrap().1)
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap());
    let distances = (distance_line.split_once(":").unwrap().1)
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap());

    times.zip(distances)
        .map(|(t, d)| RaceInfo::new(t, d))
        .collect()
}

pub fn parse_kerning(time_line: &str, distance_line: &str) -> RaceInfo {
    let duration = (time_line.split_once(":").unwrap().1)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>().unwrap();
    let record = (distance_line.split_once(":").unwrap().1)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>().unwrap();

    RaceInfo::new(duration, record)
}

#[cfg(test)]
mod tests {
    use crate::RaceInfo;

    fn get_cases() -> Vec<RaceInfo> {
        vec![
            RaceInfo::new(7, 9),
            RaceInfo::new(15, 40),
            RaceInfo::new(30, 200),
        ]
    }

    #[test]
    fn bracketing() {
        let races = get_cases();
        let lowest: Vec<u64>= races
            .iter()
            .map(|race| race.find_min())
            .collect();
        let highest: Vec<u64>= races
            .iter()
            .map(|race| race.find_max())
            .collect();

        assert_eq!(lowest, [2, 4, 11]);
        assert_eq!(highest, [5, 11, 19]);
    }

    #[test]
    fn distance_travelled() {
        let race = RaceInfo::new(7, 9);
        let distances: Vec<u64> = (0..10)
            .map(|t| race.distance_travelled(t))
            .collect();

        assert_eq!(
            distances,
            [0, 6, 10, 12, 12, 10, 6, 0, 0, 0]
            )
    }

    #[test]
    fn solve() {
        let cases = get_cases();
        let solutions: Vec<(u64, u64)> = cases
            .iter()
            .map(|case| case.solve())
            .collect();

        assert_eq!(solutions, vec![(2, 5), (4, 11), (11, 19)])
    }
}
