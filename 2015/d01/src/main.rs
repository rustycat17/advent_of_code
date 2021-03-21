use std::fs;

fn main() {
    // let input = "(())";

    let input = fs::read_to_string("data/input.txt")
        .expect("unable to read file data/input.txt");

    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("wrong symbol {}", char),
        }
        println!("current floor: {}", floor);
    }

    println!("final floor: {}", floor);
}
