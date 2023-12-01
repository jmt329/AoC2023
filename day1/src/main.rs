use std::fs;

// returns the calibration value by combining the first digit and the last digit to form a single two-digit number
fn get_p1_value(s : &str) -> u32 {
    let mut numbers = s.matches(char::is_numeric).map(|c| c.parse::<u32>().unwrap());
    let first = numbers.next().unwrap_or(0);
    let last = numbers.next_back().unwrap_or(first);
    (first * 10) + last
}

fn get_p2_value(s : &str) -> u32 {
    // find the smallest and largest index of a spelled out digit
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first_sp: Option<(usize, u32)> =  None;
    let mut last_sp: Option<(usize, u32)> = None;
    for d in digits {
        let sm = s.match_indices(d).next();
        let lg = s.rmatch_indices(d).next();
        if let Some(i) = sm {
            if i.0 <= first_sp.unwrap_or((usize::MAX, 0)).0 {
                first_sp = Some((i.0, digits.iter().position(|&r| r == i.1).unwrap() as u32 + 1))
            }
        };
        if let Some(i) = lg {
            if i.0 >= last_sp.unwrap_or((usize::MIN, 0)).0 {
                last_sp = Some((i.0, digits.iter().position(|&r| r == i.1).unwrap() as u32 + 1));
            }
        };
    }

    // find the smallest and largest index of each numeric digit
    let mut numbers = s.match_indices(char::is_numeric).map(|c| (c.0, c.1.parse::<u32>().unwrap()));
    let first_nm = numbers.next();
    let mut last_nm = numbers.next_back();
    if last_nm.is_none() {last_nm = first_nm};

    //let first = if first_sp.unwrap_or((usize::MAX, 0)).0 < first_nm.0 {first_sp.unwrap().1} else {first_nm.1};
    //let last = if last_sp.unwrap_or((usize::MIN, 0)).0 > last_nm.0 {last_sp.unwrap().1} else {last_nm.1};

    let first = match (first_nm, first_sp) {
        (Some(n), Some(s)) => if n.0 < s.0 {n.1} else {s.1},
        (Some(n), None) => n.1,
        (None, Some(s)) => s.1,
        _ => panic!("No digits found")
    };
    let last = match (last_nm, last_sp) {
        (Some(n), Some(s)) => if n.0 > s.0 {n.1} else {s.1},
        (Some(n), None) => n.1,
        (None, Some(s)) => s.1,
        _ => panic!("No digits found")
    };
    
    (first * 10) + last

}

fn main() {
    // read in input
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();

    let p1 : u32 = file_lines.clone().map(get_p1_value).sum();
    println!("p1: {}", p1);

    let p2 : u32 = file_lines.map(get_p2_value).sum();
    println!("p2: {}", p2);
}
