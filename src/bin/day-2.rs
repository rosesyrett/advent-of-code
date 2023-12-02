use std::fs;
use regex::Regex;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

fn main() {
    let filepath: String = String::from("inputs/day-2.txt");
    
    let contents = fs::read_to_string(filepath).expect("didnt read file");
    let result_star1: Vec<u16> = contents.split("\n").filter(|x| x.len() > 0).map(|x| parse_line_star1(x)).map(|x| x).collect();
    let result_star2: Vec<u16> = contents.split("\n").filter(|x| x.len() > 0).map(|x| parse_line_star2(x)).map(|x| x).collect();

    println!("star 1 result: {}\nstar 2 result: {}", result_star1.iter().sum::<u16>(), result_star2.iter().sum::<u16>());

}

/* Given a line like 'Game 3: 5 blue, 4 red; 1 red, 2 green',
* this function returns a tuple containing the game id and a 
* vector containing tuples of the number and colour.
* So in this case, (3, [(5, "blue"), (4, "red"), (1, "red"), (2, "green")]).
*/
fn number_colour_pairs(line: &str) -> (u16, Vec<(u16, &str)>) {
    let id_re = Regex::new(r"Game (\d+):").unwrap();
    let fields_re = Regex::new(r"([0-9]+)\s*(red|blue|green)").unwrap();

    let Some((_, [id])) =
        id_re.captures(line).map(|caps| caps.extract()) else { panic!() };

    let mut fields: Vec<(u16, &str)> = vec![];

    for (_, [number, colour]) in fields_re.captures_iter(line).map(|caps| caps.extract()) {
        fields.push((number.parse().expect("Couldn\'t parse number"), colour));
    }
    return (id.parse::<u16>().expect("Unable to parse ID"), fields)
}

/* This function takes a like like Game 3: 5 blue, 4 red; 1 red, 2 green
 * And returns the game ID (in this case, 3) if it is possible for the
 * constant number of red, blue or green boxes in the bag.
 */
fn parse_line_star1(line: &str) -> u16 {
    let (id, fields) = number_colour_pairs(line);

    for each_field in fields {
        let (number, colour) = each_field;

        let valid: bool = match colour {
            "red" => number <= RED as u16,
            "blue" => number <= BLUE as u16,
            "green" => number <= GREEN as u16,
            _ => {
                panic!("invalid colour found!");
            }
        };

        if !valid {
            return 0
        }
    }

    return id
}


/*
 * This function takes a line like Game 3: 5 blue, 4 red; 1 red, 2 green
 * And finds the minimum number of each colour possible for this game, i.e.
 * 4 red, 2 green and 5 blue. Any less reds and this would be impossible.
 * It then returns the multiple of all these numbers, i.e. 4*2*5.
 */
fn parse_line_star2(line: &str) -> u16 {
    let (_, fields) = number_colour_pairs(line);

    let mut max_red: u16 = 0;
    let mut max_green: u16 = 0;
    let mut max_blue: u16 = 0;

    for each_field in fields {
        let (number, colour) = each_field;

        match colour {
            "red" => {
                max_red = if number > max_red {number} else {max_red};
            },
            "blue" => {
                max_blue = if number > max_blue {number} else {max_blue};
            },
            "green" => {
                max_green = if number > max_green {number} else {max_green};
            },
            _ => {
                panic!("invalid colour found!");
            }
        }
    }

    return max_red*max_green*max_blue
}
