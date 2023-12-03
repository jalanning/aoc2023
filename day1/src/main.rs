use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

fn main() {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines("./day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                sum += build_int(&ip);
            }
        }
    }
    println!("The final sum is {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_int(line: &str) -> i32 {
    // let options = ["1", "2", "3", "4", "5", "6", "7", "8", "9"] // part 1
    let options = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut matches: Vec<_> = options.iter().enumerate()
                                .map(|(i, o)| 
                                    line
                                        .match_indices(o)
                                        // tranform into list of tuples (location, digit_value),
                                        // taking advantage of the relationship 
                                        // between option index and digit value
                                        .map(|l| (l.0, if i >= 9 { i - 8 } else { i + 1 }))
                                        .collect::<Vec<_>>())
                                // filter options that didn't match
                                .filter(|m| m.len() > 0)
                                .flatten()
                                .collect();
    // sort matches by location in string
    matches.sort_by(|a, b| a.0.cmp(&b.0));
    // grab first and last match and transform individual digits into two digit value
    let ans = matches[0].1 * 10 + matches[matches.len()-1].1;
    println!("{:?}, {}, {}", matches, line, ans);
    return ans.try_into().unwrap()
}
