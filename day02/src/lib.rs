use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
pub struct GameData {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl GameData {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        GameData {
            blue,
            red,
            green,
        }
    }

    pub fn from_string(st: &str) -> Result<Self> {
        let mut blue = 0usize;
        let mut red = 0usize;
        let mut green = 0usize;

        for part in st.split(", ") {
            let (num, color) = part.split_once(" ").unwrap();
            match color {
                "blue" => blue = num.parse::<usize>()?,
                "red" => red = num.parse::<usize>()?,
                "green" => green = num.parse::<usize>()?,
                _ => bail!("Unknown color {color}")
            }
        }

        Ok(GameData::new(red, green, blue))
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    draws: Vec<GameData>,
}

impl Game {
    pub fn is_possible(&self, reference: &GameData) -> bool {
        self.draws.iter()
            .all(|draw|
                 draw.blue <= reference.blue &&
                 draw.red <= reference.red &&
                 draw.green <= reference.green)
    }
}

pub fn parse_line(st: &str) -> Game {
    let (front, back) = st.split_once(": ").unwrap();
    let (_, id) = front.split_once(" ").unwrap();

    let draws = back.split("; ")
        .map(|part| GameData::from_string(part).unwrap())
        .collect::<Vec<_>>();

    Game {
        id: id.parse::<usize>().unwrap(),
        draws,
    }
}

#[cfg(test)]
mod tests {
    use crate::GameData;

    #[test]
    fn build_data_from_string() {
        assert_eq!(
            GameData::from_string("3 blue, 4 red").ok(),
            Some(GameData::new(4, 0, 3))
            );
        assert_eq!(
            GameData::from_string("1 red, 2 green, 6 blue").ok(),
            Some(GameData::new(1, 2, 6))
            );
        assert_eq!(
            GameData::from_string("2 green").ok(),
            Some(GameData::new(0, 2, 0))
            );
    }
}
