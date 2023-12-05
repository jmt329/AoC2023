use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Card {
    num: u32,
    win: HashSet<u32>,
    got: HashSet<u32>,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let mut x = s.split(&[':', '|']);
        let num = x
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let win = x
            .next()
            .unwrap()
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let got = x
            .next()
            .unwrap()
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        Self { num, win, got }
    }

    fn count_wins(&self) -> u32 {
        self.win.intersection(&self.got).count() as u32
    }

    fn count_points(&self) -> u32 {
        let c = self.win.intersection(&self.got).count();
        if c == 0 {
            0
        } else {
            1 << (c as u32 - 1)
        }
    }
}

fn main() {
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();

    let p1: u32 = file_lines
        .clone()
        .map(Card::from_str)
        .map(|c| c.count_points())
        .sum();
    println!("p1: {}", p1);

    let cards = file_lines.clone().map(Card::from_str);
    let num_cards: u32 = cards
        .fold(HashMap::new(), |mut acc: HashMap<u32, u32>, e: Card| {
            for i in 1..e.count_wins() + 1 {
                let w = acc.get(&e.num).unwrap_or(&0).to_owned() + 1; // the number of copies plus this win
                acc.entry(e.num + i)
                    .and_modify(|c: &mut u32| *c += w)
                    .or_insert(w);
            }
            acc.entry(e.num).and_modify(|c| *c += 1).or_insert(1); // add original
            acc
        })
        .values()
        .sum();
    println!("p2: {}", num_cards);
}
