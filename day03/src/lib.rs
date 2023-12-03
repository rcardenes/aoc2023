#[derive(Debug, PartialEq)]
pub struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub enum SchematicObject {
    Number { number: u32, coords: Vec<Coords> },
    Symbol { symbol: char, coords: Coords },
}

impl SchematicObject {
    fn new(members: &[char], columns: &[i32], row: i32) -> Self {
        match members[0] {
            '0'..='9' => {
                SchematicObject::Number {
                    number: String::from_iter(members).parse::<u32>().unwrap(),
                    coords: columns.iter().map(|&x| Coords { x, y: row }).collect::<Vec<_>>(),
                }
            }
            c => {
                SchematicObject::Symbol { symbol: c, coords: Coords { x: columns[0], y: row } }
            }
        }
    }

    pub fn neighbors_row(&self, row: i32) -> bool {
        let obj_row = match self {
            SchematicObject::Symbol { coords, .. } => { coords.y }
            SchematicObject::Number { coords, .. } => { coords[0].y }
        };

        (obj_row - row).abs() < 2
    }

    pub fn num(&self) -> u32 {
        match self {
            &Self::Number { number, .. } => number,
            _ => unimplemented!() // Won't happen... hopefully!
        }
    }

    // Used to compare adjacency of a Number (self) an a Symbol (other)
    pub fn adjacent_to(&self, other: &SchematicObject) -> bool {
        let my_coords = match self {
            Self::Number { coords, .. } => coords,
            _ => unimplemented!() // Not happening...
        };

        let other_coords = match other {
            Self::Symbol { coords, .. } => coords,
            _ => unimplemented!() // ...
        };

        my_coords.iter().any(|Coords { x, y }|
                             (y - other_coords.y).abs() < 2 && 
                             (x - other_coords.x).abs() < 2
                            )
    }

    pub fn is_part_num(&self, symbols: &[SchematicObject]) -> bool {
        match self {
            SchematicObject::Number { .. } => {
                symbols.iter()
                    .any(|member|
                         match member {
                             SchematicObject::Symbol { .. } => self.adjacent_to(member),
                             _ => false
                         })
            }
            _ => false
        }
    }

    pub fn is_gear(&self, parts: &[SchematicObject]) -> bool {
        match self {
            &SchematicObject::Symbol { symbol, .. } => {
                symbol == '*' &&
                    parts.iter().filter(|&part| part.adjacent_to(self)).count() == 2
            }
            _ => unimplemented!() // Won't happen... hopefully!
        }
    }

    pub fn gear_ratio(&self, parts: &[SchematicObject]) -> u32 {
        match self {
            &SchematicObject::Symbol { .. } => {
                let values = parts.iter().filter(|&part| part.adjacent_to(self)).map(|part| part.num()).collect::<Vec<_>>();
                values[0] * values[1]
            }
            _ => unimplemented!() // Won't happen... hopefully!
        }
    }
}

pub fn parse_line(line: &str, row: i32) -> Vec<SchematicObject> {
    let mut result = vec![];
    let mut members = vec![];
    let mut columns = vec![];
    for (col, ch) in line.chars().enumerate() {
        match ch {
            '.' => {
                if members.len() > 0 {
                    result.push(SchematicObject::new(&members, &columns, row));
                    members.clear();
                    columns.clear();
                }
            }
            '0'..='9' => {
                members.push(ch);
                columns.push(col as i32);
            }
            c => {
                if members.len() > 0 {
                    result.push(SchematicObject::new(&members, &columns, row));
                    members.clear();
                    columns.clear();
                }
                result.push(SchematicObject::new(&[c], &[col as i32], row));
            }
        }
    }

    if members.len() > 0 {
        result.push(SchematicObject::new(&members, &columns, row));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use crate::parse_line;
    use crate::{Coords, SchematicObject};

    #[test]
    fn parsing() {
        assert_eq!(
            parse_line("467..114..", 0),
            vec![
                SchematicObject::Number { number: 467, coords: vec![Coords { x: 0, y: 0 }, Coords { x: 1, y: 0 }, Coords { x: 2, y: 0}] },
                SchematicObject::Number { number: 114, coords: vec![Coords { x: 5, y: 0 }, Coords { x: 6, y: 0 }, Coords { x: 7, y: 0}] },
            ]);
        assert_eq!(
            parse_line("...*......", 1),
            vec![
                SchematicObject::Symbol { symbol: '*', coords: Coords { x: 3, y: 1 } }
            ]);
        assert_eq!(
            parse_line("......*617", 4),
            vec![
                SchematicObject::Symbol { symbol: '*', coords: Coords { x: 6, y: 4 } },
                SchematicObject::Number { number: 617, coords: vec![Coords { x: 7, y: 4 }, Coords { x: 8, y: 4 }, Coords { x: 9, y: 4}] },
            ]);
    }

    #[test]
    fn is_part() {
        let symbols = vec![
                SchematicObject::Symbol { symbol: '*', coords: Coords { x: 3, y: 1 } },
                SchematicObject::Symbol { symbol: '*', coords: Coords { x: 6, y: 4 } },
        ];
        let num1 = SchematicObject::Number {
            number: 617,
            coords: vec![Coords { x: 7, y: 4 }, Coords { x: 8, y: 4 }, Coords { x: 9, y: 4}]
        };
        let num2 = SchematicObject::Number {
            number: 58,
            coords: vec![Coords { x: 8, y: 5 }, Coords { x: 9, y: 5 }],
        };

        assert!(num1.is_part_num(&symbols));
        assert!(!num2.is_part_num(&symbols));
    }
}
