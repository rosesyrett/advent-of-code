use regex::{RegexSet, Regex};
use std::fs;

const PATTERNS: [&str; 10] = [
    r"\d{1}",
    r"one",
    r"two",
    r"three",
    r"four",
    r"five",
    r"six",
    r"seven",
    r"eight",
    r"nine"
];

fn main() {

    // Both patterns will match different ranges of this string.
    let filepath: String = String::from("calibration.txt");

    
    let contents = fs::read_to_string(filepath).expect("didnt read file");
    let result: Vec<u32> = contents.split("\n").filter(|x| x.len() > 0).map(|x| parse_line(x)).collect();

    println!("{}", result.iter().sum::<u32>());

}

fn parse_line(line: &str) -> u32 {
    // Compile a set matching any of our patterns.
    let set = RegexSet::new(PATTERNS).unwrap();
    // Compile each pattern independently.
    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    // Match against the whole set first and identify the individual
    // matching patterns.

    let matches_iter = set.matches(line).into_iter().map(|index| &regexes[index]);

    // add start and end indices to lists,
    let mut start_indices = Vec::new();
    let mut stop_indices = Vec::new();
    for re in matches_iter {
        for m in re.find_iter(line) {
            start_indices.push(m.start());
            stop_indices.push(m.end());
        }
    }

    let max_s_pos = start_indices.iter().position(|x| x == start_indices.iter().max().unwrap());
    let min_s_pos = start_indices.iter().position(|x| x == start_indices.iter().min().unwrap());
    //just find the indices 

    let min_value = &line[start_indices[min_s_pos.unwrap()]..stop_indices[min_s_pos.unwrap()]];
    let max_value = &line[start_indices[max_s_pos.unwrap()]..stop_indices[max_s_pos.unwrap()]];

    //finally, check the length of the values. if more than 1, convert it to string digit.

    let mut first_digit = min_value.to_string();
    let mut last_digit = max_value.to_string();
    if min_value.len() > 1 {
        first_digit = PATTERNS.iter().position(|x| *x == min_value).expect("Invalid digit").to_string();
    }
    if max_value.len() > 1 {
        last_digit = PATTERNS.iter().position(|x| *x == max_value).expect("Invalid digit").to_string();
    }

    return (first_digit + &last_digit).parse::<u32>().unwrap();
}
