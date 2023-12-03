use std::io::stdin;

use day03::{parse_line, SchematicObject, Symbol, Number};

fn main() {
    let mut lines = stdin().lines().enumerate();
    let mut maybe_gears: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    while let Some((row, Ok(line))) = lines.next() {
        let row = row as i32;

        for object in parse_line(&line, row as i32) {
            match object {
                SchematicObject::Numeric(num) => {
                    numbers.push(num)
                }
                SchematicObject::Symbolic(sym) => {
                    if sym.maybe_gear() {
                        maybe_gears.push(sym);
                    }
                }
            }
        }
    }

    numbers.retain(|number| number.is_part_num(&maybe_gears));
    let ratios = maybe_gears.iter()
        .filter_map(|mg| mg.gear_ratio(&numbers));

    println!("The added ratios are: {}", ratios.sum::<u32>());
}
