use std::fs;

fn main() {
    // let input = "(())";

    let input = fs::read_to_string("data/input.txt").expect("unable to read file data/input.txt");

    let mut floor = 0;
    let mut first_basement = 0;

    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("wrong symbol {}", char),
        }

        if floor == -1 && first_basement == 0 {
            first_basement = index + 1;
        }
    }

    println!("first time in the first basement: {}", first_basement);
    println!("final floor: {}", floor);
}
