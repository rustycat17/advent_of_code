use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    let mut coord1 = Point { x: 0, y: 0 };
    let mut coord2 = Point { x: 0, y: 0 };

    let mut unique_locations = vec![coord1];

    for (index, symbol) in input.chars().enumerate() {
        // println!("{}", symbol);
        if (index + 1) % 2 != 0 {
            coord1 = parse_direction(coord1, symbol);
            if !unique_locations.contains(&coord1) {
                unique_locations.push(coord1);
            }
        } else {
            coord2 = parse_direction(coord2, symbol);
            if !unique_locations.contains(&coord2) {
                unique_locations.push(coord2);
            }
        }
    }

    println!("{}", unique_locations.len());
}

fn parse_direction(mut coord: Point, symbol: char) -> Point {
    if symbol == '>' {
        coord = coord.calc_coordinates(Point { x: 1, y: 0 });
    } else if symbol == '<' {
        coord = coord.calc_coordinates(Point { x: -1, y: 0 });
    } else if symbol == '^' {
        coord = coord.calc_coordinates(Point { x: 0, y: 1 });
    } else if symbol == 'v' {
        coord = coord.calc_coordinates(Point { x: 0, y: -1 });
    } else {
        panic!("unknown symbol: {}", symbol);
    }
    coord
}
