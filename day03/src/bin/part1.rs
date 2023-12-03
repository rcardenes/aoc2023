use std::io::stdin;

use day03::{parse_line, SchematicObject, Symbol, Number};

fn main() {
    let mut lines = stdin().lines().enumerate();
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    while let Some((row, Ok(line))) = lines.next() {
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

    let parts = numbers.iter().filter(|number| number.is_part_num(&symbols)).collect::<Vec<_>>();

    println!("The sum of all the part numbers is: {}", parts.iter().map(|part| part.value).sum::<u32>());
}
