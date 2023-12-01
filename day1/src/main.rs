use std::fs;

// returns the calibration value by combining the first digit and the last digit to form a single two-digit number
fn get_value(s : &str) -> u32{
    let mut numbers = s.matches(char::is_numeric).map(|c| c.parse::<u32>().unwrap());
    let first = numbers.next().unwrap_or(0);
    let last = numbers.next_back().unwrap_or(first);
    (first * 10) + last
}

fn main() {
    // read in input
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();

    let p1 : u32 = file_lines.map(get_value).sum();
    println!("p1: {}", p1);
}
