use std::fs;

#[derive(Debug)]
struct NumPos {
    num: u32,
    idxs: Vec<usize>,
    part: bool
}

fn get_numbers(s: &str) -> Vec<NumPos> {
    let mut ret: Vec<NumPos> = vec![];
    let mut idx_vec: Vec<usize> = vec![];
    let mut acc = 0;
    let mut last_idx = 0;
    let numpos = s.match_indices(char::is_numeric).filter_map(|(i, e)| {
        if e.parse::<u32>().is_ok() {
            Some((e.parse::<u32>().unwrap(), i))
        } else {
            None
        }
    });

    for (num, idx) in numpos {
        if (idx - last_idx) <= 1 {
            acc = (acc * 10) + num;
            idx_vec.push(idx);
            last_idx = idx;
        } else {
            if !idx_vec.is_empty() {
                ret.push(NumPos {
                    num: acc,
                    idxs: idx_vec,
                    part: false
                });
            }
            acc = num;
            idx_vec = vec![idx];
            last_idx = idx;
        }
    }
    if !idx_vec.is_empty() {
        ret.push(NumPos {
            num: acc,
            idxs: idx_vec,
            part: false
        });
    }
    ret
}

fn get_symbol_positions(s: &str) -> Vec<usize> {
    s.match_indices(|x: char| !x.is_numeric() && (x != '.')).map(|(i, _)| i).collect()
}

fn set_part(nps : &mut Vec<NumPos>, idx : usize) {
    for n in nps {
        if n.idxs.contains(&(idx - 1)) {n.part = true};
        if n.idxs.contains(&idx) {n.part = true};
        if n.idxs.contains(&(idx + 1)) {n.part = true};
    }
}

fn main() {
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();
    let mut numbers = vec![];
    let mut sympos = vec![];

    for line in file_lines {
        numbers.push(get_numbers(line));
        sympos.push(get_symbol_positions(line));
    }

    // find part numbers
    for (line_num, sym) in sympos.iter().enumerate() {
        for s in sym {
            if line_num > 0 {
                // look at line above
                set_part(&mut numbers[line_num - 1], *s);
            }
            if line_num < (numbers.len() - 1) {
                // look at line below
                set_part(&mut numbers[line_num + 1], *s);
            }
            // look at current line
            set_part(&mut numbers[line_num], *s);
        }
    }

    let p1 = numbers.iter().flatten().filter(|e| e.part).fold(0, |e, acc| e+acc.num);
    println!("p1: {}", p1)
}
