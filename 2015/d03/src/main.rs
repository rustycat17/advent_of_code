use std::fs;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn calc_coordinates(self, second_point: Point) -> Self {
        Self {
            x: self.x + second_point.x,
            y: self.y + second_point.y,
        }
    }
}

fn main() {
    // let input = "^v^v^v^v^v";
    let filename = "data/input.txt";
    let input = fs::read_to_string(filename);
    let input = match input {
        Ok(input) => input,
        Err(err) => panic!("could not open file {}, error message {}", filename, err),
    };

    let mut coord = Point {
        x: 0,
        y: 0,
    };
    let mut unique_locations = vec![coord];

    for symbol in input.chars() {
        // println!("{}", symbol);
        if symbol == '>' {
            coord = coord.calc_coordinates(Point {x: 1, y: 0});
        } else if symbol == '<' {
            coord = coord.calc_coordinates(Point {x: -1, y: 0});
        } else if symbol == '^' {
            coord = coord.calc_coordinates(Point {x: 0, y: 1});
        } else if symbol == 'v' {
            coord = coord.calc_coordinates(Point {x: 0, y: -1});
        } else {
            panic!("unknown symbol: {}", symbol);
        }
        if !unique_locations.contains(&coord) {
            unique_locations.push(coord);
        }
    }

    println!("{:?}", coord);
    println!("{:?}", unique_locations);
    println!("{}", unique_locations.len());
}
