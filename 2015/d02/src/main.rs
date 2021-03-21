fn main() {
    let file_open = std::fs::read_to_string("data/input.txt");
    let input = match file_open {
        Ok(input) => input,
        Err(err) => panic!("file not found, error code {}", err),
    };

    let mut total_sum = 0;

    for line in input.split("\n") {
        let len_vec:Vec<i32> = line
            .split("x")
            .map(|x| x.parse().unwrap())
            .collect();

        let area_vec = vec![len_vec[0] * len_vec[1], len_vec[0] * len_vec[2], len_vec[1] * len_vec[2]];

        let minimum = area_vec.iter().min();
        match minimum {
            Some(value) => value,
            None => panic!("vector is empty"),
        };

        let iter_sum = 2 * (area_vec[0] + area_vec[1] + area_vec[2]) + minimum.unwrap();

        total_sum += iter_sum;
    }

    println!("square feet of wrapping paper: {}", total_sum);
}
