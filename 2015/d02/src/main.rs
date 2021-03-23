fn main() {
    let file_open = std::fs::read_to_string("data/input.txt");
    let input = match file_open {
        Ok(input) => input,
        Err(err) => panic!("file not found, error code {}", err),
    };

    let mut total_area_paper = 0;
    let mut total_ribbon_length = 0;

    for line in input.split('\n') {
        let mut len_vec: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();

        let current_area_paper = calculate_area(&len_vec);
        let current_ribbon_length = calculate_length(&mut len_vec);

        total_area_paper += current_area_paper;
        total_ribbon_length += current_ribbon_length;
    }

    println!("square feet of wrapping paper: {}", total_area_paper);
    println!("length of ribbon: {}", total_ribbon_length);
}

fn calculate_area(len_vec: &[i32]) -> i32 {
    let area_vec = [
        len_vec[0] * len_vec[1],
        len_vec[0] * len_vec[2],
        len_vec[1] * len_vec[2],
    ];

    let min_area = area_vec.iter().min();
    match min_area {
        Some(value) => value,
        None => unreachable!("vector is empty"),
    };

    2 * (area_vec[0] + area_vec[1] + area_vec[2]) + min_area.unwrap()
}

fn calculate_length(len_vec: &mut Vec<i32>) -> i32 {
    let ribbon_bow: i32 = len_vec.iter().product();

    len_vec.sort_unstable();
    len_vec.pop();

    let ribbon_wrap = 2 * (len_vec[0] + len_vec[1]);

    ribbon_wrap + ribbon_bow
}
