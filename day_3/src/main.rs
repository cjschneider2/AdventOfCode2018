extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    pub number:    usize,
    pub top_edge:  usize,
    pub left_edge: usize,
    pub width:     usize,
    pub height:    usize,
}

fn main() {
    let input_str = include_str!("../input.txt");
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims = {
        let mut claims = Vec::new();

        for line in input_str.lines() {
            let claim = parse_line(&re, line);
            claims.push(claim);
        }

        claims
    };

    let mut map = vec![vec![0; 1000]; 1000];
    apply_claims(&claims, &mut map);

    let overlaps = overlap_count(&map);

    println!("Part 1: {}", overlaps);

    let claim_number = search_non_overlapping_claims(&claims, &map);
    println!("Part 2: {:?}", claim_number);
}

fn search_non_overlapping_claims(claims: &[Claim], map: &Vec<Vec<i32>>) -> Option<usize> {
    let mut claim_number: Option<usize> = None;

    'claim_loop: for claim in claims {
        let x = claim.left_edge;
        let y = claim.top_edge;
        for idx in 0..claim.width {
            for idy in 0..claim.height {
                let val = map[x + idx][y + idy];
                if val != 1 { continue 'claim_loop; }
            }
        }
        if claim_number.is_none() {
            claim_number = Some(claim.number);
        } else {
            panic!("Double claim numbers")
        }
    }

    claim_number
}

fn overlap_count(map: &Vec<Vec<i32>>) -> i32 {
    let mut overlaps = 0;
    for row in map {
        for val in row {
            if *val > 1 { overlaps += 1; }
        }
    }
    overlaps
}

fn apply_claims(claims: &[Claim], map: &mut Vec<Vec<i32>>) {
    for claim in claims {
        let x = claim.left_edge;
        let y = claim.top_edge;
        for idx in 0..claim.width {
            for idy in 0..claim.height {
                map[x + idx][y + idy] += 1;
            }
        }
    }
}

fn parse_line(re: &Regex, line: &str) -> Claim {
    let cap = re.captures(line).unwrap();

    let number = cap[1].parse::<usize>().unwrap();
    let left_edge = cap[2].parse::<usize>().unwrap();
    let top_edge = cap[3].parse::<usize>().unwrap();
    let width = cap[4].parse::<usize>().unwrap();
    let height = cap[5].parse::<usize>().unwrap();


    Claim {
        number,
        top_edge,
        left_edge,
        width,
        height
    }
}
