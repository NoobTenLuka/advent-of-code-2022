use itertools::Itertools;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let mut table = 0u64;
            let half = line.len() / 2;
            let mut prio = 0;
            for (i, c) in line.chars().enumerate() {
                let val = 1u64 << (c as u8 - 65);
                if i + 1 <= half {
                    table |= val;
                } else if table & val != 0 {
                    prio = (c as u8 - 38) % 53;
                    if c >= 'a' {
                        prio -= 5;
                    }
                    break;
                }
            }
            prio as i32
        })
        .sum::<i32>().to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|lines| {
            let mut table1 = 0u64;
            for c in lines.0.chars() {
                let val = 1u64 << (c as u8 - 65);
                table1 |= val;
            }
            let mut table2 = 0u64;
            for c in lines.1.chars() {
                let val = 1u64 << (c as u8 - 65);
                table2 |= val;
            }
            let badge = lines
                .2
                .chars()
                .find(|c| {
                    let val = 1u64 << (*c as u8 - 65);
                    (val & table1 & table2) != 0
                })
                .unwrap();
            let mut prio = (badge as u8 - 38) % 53;
            if badge >= 'a' {
                prio -= 5;
            }
            prio as i32
        })
        .sum::<i32>().to_string()
}
