use std::collections::HashMap;

fn main() {
    let input_str = include_str!("../input.txt");

    let (twos, threes) = part_1(&input_str);
    println!("Part 1: {}", twos * threes);

    let pt_2 = part_2(&input_str);

    println!("part 2: {}", pt_2.unwrap());
}

fn part_1(input_str: &str) -> (usize, usize) {
    let mut twos_count = 0;
    let mut threes_count = 0;

    for line in input_str.lines() {
        let mut letters: HashMap<char, usize> = HashMap::new();

        for chr in line.chars() {
            match letters.insert(chr, 1) {
                Some(val) => { letters.insert(chr, val + 1); }
                None => ()
            }
        }

        for (_key, val) in letters.iter() {
            if *val == 2 {
                twos_count += 1;
                break;
            }
        }

        for (_key, val) in letters.iter() {
            if *val == 3 {
                threes_count += 1;
                break;
            }
        }
    }

    (twos_count, threes_count)
}

fn part_2(input_str: &str) -> Option<String> {
    let (line_a, line_b) = find_diff_1(input_str)?;

    let common = line_a.chars().zip(line_b.chars())
        .filter_map(|(a, b)| {
            if a == b { Some(a) } else { None }
        })
        .collect::<String>();

    Some(common)
}

fn find_diff_1(input_str: &str) -> Option<(String, String)> {
    let lines = input_str.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();

    for id_a in 0..lines_len {
        for id_b in (id_a + 1)..lines_len {
            let mut diffs = 0;
            let line_a = lines[id_a];
            let line_b = lines[id_b];

            for (a, b) in line_a.chars().zip(line_b.chars()) {
                if a != b { diffs += 1; }
            }

            if diffs == 1 { return Some((line_a.into(), line_b.into())); }
        }
    }

    None
}
