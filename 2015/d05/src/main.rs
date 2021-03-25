use std::fs;

fn main() {
    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("could not open file, error message: {}", err),
    };

    let mut good_strings = 0;
    for line in input.split('\n') {
        good_strings = calculate_good_strings(good_strings, line)
    }

    println!("{}", good_strings);
}

fn calculate_good_strings(mut good_strings: i32, line: &str) -> i32 {
    let mut vowel_counts = 0;

    let mut last_letter = '_';
    let mut double_letter = false;

    let mut bi_string = false;

    for character in line.chars() {
        match character {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_counts += 1,
            _ => (),
        }

        if !bi_string {
            bi_string = matches!((last_letter, character), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y'));
        }

        if !double_letter {
            double_letter = character == last_letter;
        }

        last_letter = character;
    }

    if vowel_counts >= 3 && double_letter && !bi_string {
        good_strings += 1;
    }

    good_strings
}
