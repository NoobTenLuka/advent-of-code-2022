pub fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (enemy, myself) = line.split_once(" ").unwrap();
            match myself {
                "X" => {
                    1 + match enemy {
                        "A" => 3,
                        "B" => 0,
                        "C" => 6,
                        _ => unreachable!(),
                    }
                }
                "Y" => {
                    2 + match enemy {
                        "A" => 6,
                        "B" => 3,
                        "C" => 0,
                        _ => unreachable!(),
                    }
                }
                "Z" => {
                    3 + match enemy {
                        "A" => 0,
                        "B" => 6,
                        "C" => 3,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum()
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (enemy, myself) = line.split_once(" ").unwrap();
            match myself {
                "X" => {
                    0 + match enemy {
                        "A" => 3,
                        "B" => 1,
                        "C" => 2,
                        _ => unreachable!(),
                    }
                }
                "Y" => {
                    3 + match enemy {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => unreachable!(),
                    }
                }
                "Z" => {
                    6 + match enemy {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum()
}
