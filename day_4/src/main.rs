use std::collections::HashMap;

mod event;

use crate::event::parse_events;
use crate::event::EventKind;

fn main() {
    let input_str = include_str!("../input.txt");
    let event_list = parse_events(&input_str);

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut id = 0;
    let mut start = 0;

    event_list
        .iter()
        .fold((&mut map, id, start), |(map, id, start), event| {
            match event.kind {
                EventKind::WakesUp => {
                    let vec = map.get_mut(&id).unwrap();
                    for idx in start..event.minute { vec[idx] += 1; }
                    (map, id, start)
                }
                EventKind::Sleeps => {
                    (map, id, event.minute)
                }
                EventKind::BeginShift(guard) => {
                    if !map.contains_key(&guard) {
                        map.insert(guard, vec![0; 60]);
                    }
                    (map, guard, start)
                }
                _ => (map, id, start),
            }
        });

    let mut part_1 = (0, 0, 0); // (id, sleep_minutes, max_minute)
    let mut part_2 = (0, 0, 0); // (id, minute, max)

    for (id, vec) in map.iter() {
        //println!("id: {:4}, {:2?}", id, vec);
        let sum = vec.iter().fold(0, |acc, x| acc + x);
        let (idx, max) = vec.iter()
            .enumerate()
            .fold((0, 0), |(idx, max), (i, &v)| {
                if v > max { (i, v) } else { (idx, max) }
            });

        if sum > part_1.1 {
            let (index, _max_minute) = vec.iter()
                .enumerate()
                .max_by(|&(_, &a), &(_, b)| a.cmp(b))
                .unwrap();
            part_1 = (*id, sum, index);
        }

        if max > part_2.2 {
            part_2 = (*id, idx, max)
        }
    }

    println!("part 1: {:?} => {}", part_1, part_1.0 * part_1.2);
    println!("part 2: {:?} => {}", part_2, part_2.0 * part_2.1);

    assert_eq!(151754, part_1.0 * part_1.2);
    assert_eq!(19896, part_2.0 * part_2.1);
}
