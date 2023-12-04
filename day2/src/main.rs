use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn index_if_possible_else_0_pt1(line: &str, maximums: &HashMap<String, i32>) -> i32 {
    let mut split = line.split(":");
    let mut game = split.next().unwrap().split(" ");
    game.next();
    let game_index = game.next().unwrap().parse::<i32>().unwrap();
    let mut rounds = split.next().unwrap().split(";");
    if rounds.any(|r| 
        r.split(',').any(|c| {
            let mut csplit = c.split(" ").filter(|s| s.len() > 0);
            let num = csplit.next().unwrap().parse::<i32>().unwrap();
            let color = csplit.next().unwrap();
            return num > *maximums.get(color).unwrap();
        })
    ) {return 0};
    
    return game_index;
}

fn get_game_power(line: &str) -> i32 {
    let mut split = line.split(":");
    split.next();
    let rounds = split.next().unwrap().split(";");
    let mut maximums: HashMap<String, i32> = HashMap::new();
    for round in rounds {
        let colors = round.split(",");
        for c in colors {
            let mut csplit = c.split(" ").filter(|s| s.len() > 0);
            let num = csplit.next().unwrap().parse::<i32>().unwrap();
            let color = csplit.next().unwrap();
            if !maximums.contains_key(color) || maximums.get(color).unwrap() < &num {
                maximums.insert(color.to_string(), num);
            }
        }
    }

    return maximums.values().fold(1,|a,b| a*b);
}

fn main() {
    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    let mut maximums = HashMap::new();
    maximums.insert("red".to_string(), 12);
    maximums.insert("green".to_string(), 13);
    maximums.insert("blue".to_string(), 14);
    

    if let Ok(lines) = read_lines("./day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                sum1 += index_if_possible_else_0_pt1(&ip, &maximums);
                sum2 += get_game_power(&ip);
            }
        }
    }
    println!("Pt1: {}, Pt2: {}", sum1, sum2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}