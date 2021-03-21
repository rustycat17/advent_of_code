use std::fs;

fn main() {
    // let input = "(())";

    let input = fs::read_to_string("data/input.txt").expect("unable to read file data/input.txt");

    let mut floor = 0;
    let mut first_basement = None;

    for (index, symbol) in input.chars().enumerate() {
        match symbol {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("wrong symbol {}", symbol),
        }

        if floor == -1 && first_basement.is_none() {
            first_basement = Some(index + 1);
        }
    }

    println!("final floor: {}", floor);
    if first_basement.is_none() {
        println!("Santa was never in the first basement.")
    } else {
        println!("first time in the first basement: {}", first_basement.unwrap());
    }
}
