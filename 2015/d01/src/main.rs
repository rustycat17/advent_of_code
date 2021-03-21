use std::fs;

fn main() {
    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);

    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("{}: {}", err, filepath),
    };

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
