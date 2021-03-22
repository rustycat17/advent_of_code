fn main() {
    let file_open = std::fs::read_to_string("data/input.txt");
    let input = match file_open {
        Ok(input) => input,
        Err(err) => panic!("file not found, error code {}", err),
    };

    let mut total_area_sum = 0;

    for line in input.split("\n") {
        let len_vec:Vec<i32> = line
            .split("x")
            .map(|x| x.parse().unwrap())
            .collect();

        let current_area_sum = calculate_area(len_vec);

        total_area_sum += current_area_sum;
    }

    println!("square feet of wrapping paper: {}", total_area_sum);
}

fn calculate_area(len_vec: Vec<i32>) -> i32 {
    let area_vec = [len_vec[0] * len_vec[1], len_vec[0] * len_vec[2], len_vec[1] * len_vec[2]];

    let min_area = area_vec.iter().min();
    match min_area {
        Some(value) => value,
        None => panic!("vector is empty"),
    };

    2 * (area_vec[0] + area_vec[1] + area_vec[2]) + min_area.unwrap()

}
