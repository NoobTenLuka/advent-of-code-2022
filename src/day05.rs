use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut crate_matrix: [Vec<u8>; 9] = std::array::from_fn(|_| Vec::new());
    for (i, line) in input.lines().enumerate() {
        if i < 8 {
            for (j, box_index) in (1..=33).step_by(4).enumerate() {
                let char = line.as_bytes()[box_index];
                if char == 32 {
                    continue;
                }
                crate_matrix[j].push(char);
            }
        } else if i == 8 {
            crate_matrix.iter_mut().for_each(|elem| elem.reverse());
        } else if i >= 10 {
            let (amount, from, to) = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            for _ in 0..amount {
                let elem = crate_matrix[from - 1].pop().unwrap();
                crate_matrix[to - 1].push(elem);
            }
        }
    }
    crate_matrix
        .map(|vec| char::from_u32(*vec.last().unwrap() as u32).unwrap())
        .iter()
        .collect::<String>()
}

pub fn part2(input: String) -> String {
    let mut crate_matrix: [Vec<u8>; 9] = std::array::from_fn(|_| Vec::new());
    for (i, line) in input.lines().enumerate() {
        if i < 8 {
            for (j, box_index) in (1..=33).step_by(4).enumerate() {
                let char = line.as_bytes()[box_index];
                if char == 32 {
                    continue;
                }
                crate_matrix[j].push(char);
            }
        } else if i == 8 {
            crate_matrix.iter_mut().for_each(|elem| elem.reverse());
        } else if i >= 10 {
            let (amount, from, to) = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let mut tmp = Vec::new();
            for _ in 0..amount {
               let elem = crate_matrix[from - 1].pop().unwrap();
               tmp.push(elem);
            }
            tmp.reverse();
            crate_matrix[to - 1].append(&mut tmp);
        }
    }
    crate_matrix
        .map(|vec| char::from_u32(*vec.last().unwrap() as u32).unwrap())
        .iter()
        .collect::<String>()
}
