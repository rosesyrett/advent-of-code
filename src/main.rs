use std::fs;

fn main() {
    let filepath: String = String::from("calibration.txt");

    
    let contents = fs::read_to_string(filepath).expect("didnt read file");
    let result: Vec<u32> = contents.split("\n").filter(|x| x.len() > 0).map(|x| parse_line(x)).collect();

    println!("{}", result.iter().sum::<u32>());
}

fn parse_line(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    let mut left = line.chars();

    let mut value = left.next();
    while first_digit.is_none() && value.is_some() {
        let char_as_digit = value.unwrap().to_digit(10);

        if char_as_digit.is_some(){
            first_digit = char_as_digit;
            break;
        }

        value = left.next();
    }

    value = left.next_back();
    while last_digit.is_none() && value.is_some() {
        let char_as_digit = value.unwrap().to_digit(10);

        if char_as_digit.is_some(){
            last_digit = char_as_digit;
            break;
        }

        value = left.next_back();
    }
    
    if first_digit.is_some() {
        if last_digit.is_none() {
            let both_digits = first_digit.unwrap().to_string().repeat(2);
            return both_digits.parse::<u32>().unwrap();
        }
        else {
            let both_digits = first_digit.unwrap().to_string() + &last_digit.unwrap().to_string();
            return both_digits.parse::<u32>().unwrap();
        }
    }
    panic!();
}
