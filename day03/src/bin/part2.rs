use std::io::stdin;

use day03::{parse_line, SchematicObject};

fn main() {
    let mut lines = stdin().lines().enumerate();
    let mut maybe_gears: Vec<SchematicObject> = vec![];
    let mut numbers: Vec<SchematicObject> = vec![];

    while let Some((row, Ok(line))) = lines.next() {
        let row = row as i32;

        for object in parse_line(&line, row as i32) {
            match object {
                SchematicObject::Number { .. } => {
                    numbers.push(object)
                }
                SchematicObject::Symbol { symbol, .. } => {
                    if symbol == '*' {
                        maybe_gears.push(object);
                    }
                }
            }
        }
    }

    numbers.retain(|number| number.is_part_num(&maybe_gears));
    let gears = maybe_gears.iter().filter(|mg| mg.is_gear(&numbers)).collect::<Vec<_>>();

    println!("The added ratios are: {:#?}", gears.iter().map(|gear| gear.gear_ratio(&numbers)).sum::<u32>());
}
