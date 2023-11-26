mod day01;
mod day02;

use std::{env, fs};

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("Not a valid day!")
        .parse()
        .expect("Not a valid day!");

    let part1 = match env::args().nth(2).expect("Not a valid part!").as_str() {
        "1" => true,
        "2" => false,
        _ => panic!("Not a valid part!"),
    };

    let input = fs::read_to_string(format!("inputs/{:02}-input.txt", day)).expect("Day is invalid");

    match day {
        1 => {
            if part1 {
                day01::part1(input)
            } else {
                day01::part2(input)
            }
        }
        _ => panic!("Either invalid or not implemented"),
    }
}
