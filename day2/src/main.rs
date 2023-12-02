use std::fs;

#[derive(Debug, Clone)]
struct Cubes {
    red : u32,
    green : u32,
    blue : u32
}

impl Cubes {
    fn from_vec(v: &Vec<&str>) -> Self {
        let mut ret = Self{red: 0, green: 0, blue: 0};
        for h in v {
            let mut c = h.split_whitespace();
            let (num, color) = (c.next().unwrap().parse().unwrap(), c.next().unwrap());
            match color {
                "red" => ret.red = num,
                "green" => ret.green = num,
                "blue" => ret.blue = num,
                _ => panic!("unrecognized color")
            }
        }
        ret
    }

    fn possible_bag(&self, other: &Cubes) -> bool {
        (self.red <= other.red) && (self.green <= other.green) && (self.blue <= other.blue)
    }

    fn min_bag(lhs: &Cubes, rhs: &Cubes) -> Self {
        Self{
            red: std::cmp::max(lhs.red, rhs.red),
            green: std::cmp::max(lhs.green, rhs.green),
            blue: std::cmp::max(lhs.blue, rhs.blue)
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_line(draws: &str) -> Vec<Cubes> {
    let hands: Vec<Vec<&str>> = draws.split("; ").map(|h| h.split(", ").collect::<Vec<&str>>()).collect();
    let cubes: Vec<Cubes> = hands.iter().map(|h| Cubes::from_vec(h)).collect();
    cubes
}

fn main() {
    // read in input
    let file_string = fs::read_to_string("input").unwrap();
    let file_lines = file_string.lines();

    let mut id_acc: u32 = 0; // accumulator for ids for games that are possible
    let mut power_acc = 0;
    for line in file_lines {
        // parse each game
        let mut l = line.split(": ");
        let (game_info, draws) = (l.next().unwrap(), l.next().unwrap());
        let num : u32 = game_info.matches(char::is_numeric).collect::<String>().parse().unwrap();
        let game = parse_line(draws);
        let possible = game.clone().iter().map(|h| h.possible_bag(&Cubes{red: 12, green: 13, blue: 14})).reduce(|acc, e| acc && e).unwrap();
        if possible {id_acc += num;}
        power_acc += game.iter().fold(Cubes{red: 0, green:0, blue: 0}, |acc, e| Cubes::min_bag(&acc, e)).power();
    }
    println!("p1: {}", id_acc);
    println!("p2: {}", power_acc);
}
