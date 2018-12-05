use std::collections::HashSet;

fn main() {
    let input_str = include_str!("../input.txt");

    let part_1_answer = part_1(&input_str);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input_str);
    println!("Part 2: {}", part_2_answer);
}

fn part_1(input_str: &str) -> i32 {
    let mut frequency = 0;
    for line in input_str.lines() {
        let delta_frequency: i32 = line.parse().unwrap();
        frequency += delta_frequency;
    }
    frequency
}

fn part_2(input_str: &str) -> i32 {
    let mut freq_set = HashSet::new();

    let mut frequency = 0;
    freq_set.insert(frequency);

    loop {
        for line in input_str.lines() {
            let delta_frequency: i32 = line.parse().unwrap();
            frequency += delta_frequency;
            let new_entry = freq_set.insert(frequency);
            if !new_entry { return frequency; }
        }
    }
}
