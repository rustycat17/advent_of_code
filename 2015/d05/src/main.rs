use std::fs;

fn main() {
    let filepath = "data/input.txt";
    let input = fs::read_to_string(filepath);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("could not open file, error message: {}", err),
    };

    let mut count_good_strings = 0;
    for line in input.split('\n') {
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
                bi_string = match (last_letter, character) {
                    ('a', 'b') => true,
                    ('c', 'd') => true,
                    ('p', 'q') => true,
                    ('x', 'y') => true,
                    _ => false,
                };
            }

            if !double_letter {
                double_letter = character == last_letter;
            }

            last_letter = character;
        }

        if vowel_counts >= 3 && double_letter && !bi_string {
            count_good_strings += 1;
        }
    }

    println!("{}", count_good_strings);

}
