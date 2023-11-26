pub fn part1(input: String) -> i32 {
    let mut max_calories = -1;
    input
        .split("\n\n")
        .map(|elve| elve.lines().map(|x| x.parse::<i32>().unwrap()).sum())
        .for_each(|x| {
            if x > max_calories {
                max_calories = x;
            }
        });
    max_calories
}

pub fn part2(input: String) -> i32 {
    let mut max_calories = [-1i32; 3];
    input
        .split("\n\n")
        .map(|elve| elve.lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .for_each(|x| {
            if x > max_calories[0] {
                max_calories[2] = max_calories[1];
                max_calories[1] = max_calories[0];
                max_calories[0] = x;
            }
            else if x > max_calories[1] {
                max_calories[2] = max_calories[1];
                max_calories[1] = x;
            }
            else if x > max_calories[2] {
                max_calories[2] = x;
            }
        });
    max_calories.iter().sum()
}
