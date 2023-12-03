#[derive(Debug, PartialEq)]
struct Coords {
    col: i32,
    row: i32,
}

#[derive(Debug, PartialEq)]
struct CoordRange {
    left_col: i32,
    right_col: i32,
    row: i32,
}

impl CoordRange {
    fn new(cols: &[i32], row: i32) -> Self {
        CoordRange {
            left_col: cols[0],
            right_col: *cols.last().unwrap(),
            row
        }
    }

    fn adjacent_to(&self, point: &Coords) -> bool {
        (self.row - point.row).abs() < 2 &&
            (self.left_col..=self.right_col).any(|col| (col - point.col).abs() < 2)
    }
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: u32,
    coords: CoordRange,
}

impl Number {
    fn new(value: u32, cols: &[i32], row: i32) -> Self {
        Number {
            value,
            coords: CoordRange::new(cols, row),
        }
    }

    pub fn is_part_num(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|symbol| self.adjacent_to(symbol))
    }

    fn adjacent_to(&self, symbol: &Symbol) -> bool {
        self.coords.adjacent_to(&symbol.coords)
    }
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    ch: char,
    coords: Coords,
}

impl Symbol {
    pub fn new(ch: char, col: i32, row: i32) -> Self {
        Symbol {
            ch,
            coords: Coords { col, row },
        }
    }

    pub fn maybe_gear(&self) -> bool {
        self.ch == '*'
    }

    pub fn gear_ratio(&self, parts: &[Number]) -> Option<u32> {
        if !self.maybe_gear() { return None };

        let adjacent_parts = parts.iter()
            .filter(|&part| part.adjacent_to(self))
            .map(|part| part.value )
            .collect::<Vec<_>>();
        if adjacent_parts.len() != 2 { return None };

        Some(adjacent_parts[0] * adjacent_parts[1])
    }
}

#[derive(Debug, PartialEq)]
pub enum SchematicObject {
    Numeric(Number),
    Symbolic(Symbol),
}

impl SchematicObject {
    fn new(members: &[char], columns: &[i32], row: i32) -> Self {
        match members[0] {
            '0'..='9' => {
                SchematicObject::Numeric(
                    Number::new(String::from_iter(members).parse::<u32>().unwrap(), columns, row))
            }
            c => {
                SchematicObject::Symbolic(Symbol { ch: c, coords: Coords { col: columns[0], row } })
            }
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
    use crate::{parse_line, Number, SchematicObject, Symbol};

    const SAMPLE_INPUT: &str = include_str!("../input.small");

    #[test]
    fn parsing() {
        assert_eq!(
            parse_line("467..114..", 0),
            vec![
                SchematicObject::Numeric(Number::new(467, &[0, 1, 2], 0)),
                SchematicObject::Numeric(Number::new(114, &[5, 6, 7], 0)),
            ]);
        assert_eq!(
            parse_line("...*......", 1),
            vec![
                SchematicObject::Symbolic(Symbol::new('*', 3, 1)),
            ]);
        assert_eq!(
            parse_line("......*617", 4),
            vec![
                SchematicObject::Symbolic(Symbol::new('*', 6, 4)),
                SchematicObject::Numeric(Number::new(617, &[7, 8, 9], 4))
            ]);
    }

    #[test]
    fn is_part() {
        let symbols = vec![
            Symbol::new('*', 3, 1),
            Symbol::new('*', 6, 4),
        ];

        let num1 = Number::new(617, &[7, 8, 9], 4);
        let num2 = Number::new(58, &[8, 9], 5);

        assert!(num1.is_part_num(&symbols));
        assert!(!num2.is_part_num(&symbols));
    }

    fn collect_objects(source: &str) -> (Vec<Symbol>, Vec<Number>) {
        let mut numbers = vec![];
        let mut symbols = vec![];
        let mut lines = source.lines().enumerate();

        while let Some((row, line)) = lines.next() {
            let row = row as i32;

            for object in parse_line(&line, row as i32) {
                match object {
                    SchematicObject::Numeric(num) => {
                        numbers.push(num)
                    }
                    SchematicObject::Symbolic(sym) => {
                        symbols.push(sym);
                    }
                }
            }
        }

        (symbols, numbers)
    }

    #[test]
    fn identify_parts() {
        // WARNING: this test is based on the sample input, which doesn't cover ALL test
        // cases. Crucially, there are only two numbers that are not adjacent to symbols,
        // and they're both TO THE RIGHT of any relevant symbol (within 1 row). This
        // allows false positives with certain wrong implementation of the adjacency test.
        //
        // Figured it out the hard way... Luckily, only when refactoring (*sigh*). Well,
        // moving on...
        let (symbols, numbers) = collect_objects(SAMPLE_INPUT);
        let mut parts = numbers;

        parts.retain(|number| number.is_part_num(&symbols));

        let test_parts = vec![
            Number::new(467, &[0, 1, 2], 0),
            Number::new(35, &[2, 3], 2),
            Number::new(633, &[6, 7, 8], 2),
            Number::new(617, &[0, 1, 2], 4),
            Number::new(592, &[2, 3, 4], 6),
            Number::new(755, &[6, 7, 8], 7),
            Number::new(664, &[1, 2, 3], 9),
            Number::new(598, &[5, 6, 7], 9),
        ];

        assert_eq!(parts, test_parts);
    }

    #[test]
    fn identify_gears() {
        let (symbols, numbers) = collect_objects(SAMPLE_INPUT);
        let parts = numbers.into_iter()
            .filter(|num| num.is_part_num(&symbols))
            .collect::<Vec<_>>();

        let gears = symbols.into_iter()
            .filter(|sym| sym.gear_ratio(&parts).is_some())
            .collect::<Vec<_>>();

        let test_gears = vec![
            Symbol::new('*', 3, 1),
            Symbol::new('*', 5, 8),
        ];

        assert_eq!(gears, test_gears);
    }
}
