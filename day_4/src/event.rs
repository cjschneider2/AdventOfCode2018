use regex::Regex;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum EventKind {
    WakesUp,
    Sleeps,
    BeginShift(usize),
    Raw(String),
}

#[derive(Eq, Debug)]
pub struct Event {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub kind: EventKind,
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.year.cmp(&other.year)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
            .then_with(|| self.hour.cmp(&other.hour))
            .then_with(|| self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_events(input_str: &str) -> Vec<Event> {
    let re = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] (.+)").unwrap();
    let re_kind = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut list = input_str.lines()
        .filter_map(|line| re.captures(line))
        .map(|cap| {
            Event {
                year: cap[1].parse().unwrap(),
                month: cap[2].parse().unwrap(),
                day: cap[3].parse().unwrap(),
                hour: cap[4].parse().unwrap(),
                minute: cap[5].parse().unwrap(),
                kind: parse_event_kind(&cap[6], &re_kind),
            }
        })
        .collect::<Vec<Event>>();

    list.sort();
    list
}

fn parse_event_kind(input_str: &str, re_kind: &Regex) -> EventKind {
    match input_str {
        "wakes up" => EventKind::WakesUp,
        "falls asleep" => EventKind::Sleeps,
        _ => {
            match re_kind.captures(input_str) {
                Some(cap) => EventKind::BeginShift(cap[1].parse().unwrap()),
                None => EventKind::Raw(input_str.into())
            }
        }
    }
}
