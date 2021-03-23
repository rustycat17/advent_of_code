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

    let mut coord1 = Point {
        x: 0,
        y: 0,
    };
    let mut coord2 = Point {
        x: 0,
        y: 0,
    };

    let mut unique_locations = vec![coord1];

    for (index, symbol) in input.chars().enumerate() {
        // println!("{}", symbol);
        if (index + 1) % 2 != 0 {
            if symbol == '>' {
                coord1 = coord1.calc_coordinates(Point {x: 1, y: 0});
            } else if symbol == '<' {
                coord1 = coord1.calc_coordinates(Point {x: -1, y: 0});
            } else if symbol == '^' {
                coord1 = coord1.calc_coordinates(Point {x: 0, y: 1});
            } else if symbol == 'v' {
                coord1 = coord1.calc_coordinates(Point {x: 0, y: -1});
            } else {
                panic!("unknown symbol: {}", symbol);
            }
            if !unique_locations.contains(&coord1) {
                unique_locations.push(coord1);
            }
        } else {
            if symbol == '>' {
                coord2 = coord2.calc_coordinates(Point {x: 1, y: 0});
            } else if symbol == '<' {
                coord2 = coord2.calc_coordinates(Point {x: -1, y: 0});
            } else if symbol == '^' {
                coord2 = coord2.calc_coordinates(Point {x: 0, y: 1});
            } else if symbol == 'v' {
                coord2 = coord2.calc_coordinates(Point {x: 0, y: -1});
            } else {
                panic!("unknown symbol: {}", symbol);
            }
            if !unique_locations.contains(&coord2) {
                unique_locations.push(coord2);
            }
        }
        println!("symbol: {:?}", symbol);
        println!("index+1: {}", index + 1);
        println!("point 1: {:?}", coord1);
        println!("point 2: {:?}", coord2);

    }

    // println!("{:?}", coord);
    println!("{:?}", unique_locations);
    println!("{}", unique_locations.len());
}
