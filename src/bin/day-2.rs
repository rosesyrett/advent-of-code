use std::fs;
use regex::Regex;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

fn main() {
    let filepath: String = String::from("inputs/day-2.txt");
    
    let contents = fs::read_to_string(filepath).expect("didnt read file");
    let result: Vec<u16> = contents.split("\n").filter(|x| x.len() > 0).map(|x| parse_line(x)).map(|x| x).collect();

    println!("{}", result.iter().sum::<u16>());

}

/*
 * This function takes a like like Game 3: 5 blue, 4 red; 1 red, 2 green
 * And returns the game ID (in this case, 3) if it is possible for the
 * constant number of red, blue or green boxes in the bag.
 */
fn parse_line(line: &str) -> u16 {
    let colon_split: Vec<&str> = line.split(':').collect();

    // check the line only has one colon
    assert_eq!(colon_split.len(), 2);

    let err = format!("Splitting line {} failed", line);
    let game_id = *colon_split.get(0).unwrap();
    let id = (
        *game_id.split(' ').collect::<Vec<&str>>()
        .get(1).expect(&err)
    )
    .parse::<u16>().expect(&err);

    let sub_games = *colon_split.get(1).unwrap();

    let re = Regex::new(r"(\d+ (green|red|blue))").unwrap();
    let matches: Vec<&str> = re.find_iter(sub_games).map(|x| x.as_str()).collect();

    for each_match in matches {
        let num_color: Vec<&str> = each_match.split(" ").collect();
        //now just need to check if the number is less than the max for that colour...
        let number = (
            *num_color.get(0)
            .expect("Unable to split this match!")
        ).parse::<u16>()
        .expect("Unable to parse as int");

        let colour = *num_color
            .get(1)
            .expect("Unable to split this match!");

        let valid: bool = match colour {
            "red" => number <= RED as u16,
            "blue" => number <= BLUE as u16,
            "green" => number <= GREEN as u16,
            _ => {
                panic!("invalid colour found!");
            }
        };

        if !valid {
            return 0;
        }
    }

    return id;
}
