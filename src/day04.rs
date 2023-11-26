use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .filter(|line| {
            let replaced = line.replace(",", "-");
            let (left_min, left_max, right_min, right_max) = replaced
                .split("-")
                .map(|num| num.parse::<i32>().unwrap())
                .next_tuple()
                .unwrap();
            (left_min <= right_min && left_max >= right_max)
                || (left_min >= right_min && left_max <= right_max)
        })
        .count().to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .filter(|line| {
            let replaced = line.replace(",", "-");
            let (left_min, left_max, right_min, right_max) = replaced
                .split("-")
                .map(|num| num.parse::<i32>().unwrap())
                .next_tuple()
                .unwrap();
            right_min <= left_max && left_min <= right_max 
        })
        .count().to_string()
}
