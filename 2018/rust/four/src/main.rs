use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq)]
enum Action {
    Begin,
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Record {
    guard: u16,
    hour: u16,
    minute: u16,
    action: Action,
}

const RECORD_RE: &str = concat!(
    r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] ",
    r"(falls asleep|wakes up|Guard #(\d+) begins shift)"
);

fn parse(s: &str) -> Record {
    lazy_static! {
        static ref re: Regex = Regex::new(RECORD_RE).unwrap();
    }
    let caps = re.captures(s).unwrap();
    Record {
        hour: caps[1].parse::<u16>().unwrap(),
        minute: caps[2].parse::<u16>().unwrap(),
        guard: if let Some(g) = caps.get(4) {
            g.as_str().parse::<u16>().unwrap()
        } else {
            0
        },
        action: match &caps[3][..5] {
            "falls" => Action::Sleep,
            "wakes" => Action::Wake,
            _ => Action::Begin,
        },
    }
}

fn load_records() -> Vec<Record> {
    let mut lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .lines()
        .map(|s| s.to_string())
        .collect();
    lines.sort();

    let mut records: Vec<Record> = vec![];
    let mut g = 0;

    for l in lines {
        let mut r = parse(&l);
        if r.guard == 0 {
            r.guard = g;
        } else {
            g = r.guard;
        }
        records.push(r);
    }
    records
}

// each guard gets a vector of 60 integers 0-59, representing the number of minutes
// total asleep at the given minute.
fn create_time_map(records: &Vec<Record>) -> HashMap<u16, [u8; 60]> {
    let mut m: HashMap<u16, [u8; 60]> = HashMap::new();
    let mut a = 0;
    for r in records.iter().filter(|r| r.action != Action::Begin) {
        if r.action == Action::Sleep {
            a = r.minute;
        } else if r.action == Action::Wake {
            let minutes = m.entry(r.guard).or_insert([0u8; 60]);
            for i in a..r.minute {
                minutes[i as usize] += 1
            }
        }
    }
    m
}

// find the guard with the most minutes sleeping
// return the guard id * the minute that guard is most asleep
fn part1(time_map: &HashMap<u16, [u8; 60]>) -> u32 {
    // make a list of (sum, guard_id) tuples
    let mut v: Vec<(u32, u16)> = time_map
        .iter()
        .map(|(k, v)| (v.iter().fold(0u32, |acc, x| acc + *x as u32), *k))
        .collect();
    // sort it descending by first element
    v.sort_by_key(|t| t.0);
    // sum, guardid of the sleepiest is at the end:
    let (_s, g) = v[v.len() - 1];
    //println!("Guard {} slept for {} minutes", g, s);
    let tm = time_map[&g];
    let mx = tm.iter().max().unwrap();
    let idx = tm.iter().position(|&x| x == *mx).unwrap();
    //println!("max time was {} minutes at minute {}", mx, idx);
    (idx as u32) * (g as u32)
}

// find the guard that is most frequently asleep on a single minute
// return the id of that guard * that minute
fn part2(time_map: &HashMap<u16, [u8; 60]>) -> u32 {
    let mut mx: u8 = 0;
    let mut gid: u16 = 0;
    let mut idx = 0;
    for (g, tm) in time_map {
        let x = tm.iter().max().unwrap();
        if *x > mx {
            mx = *x;
            gid = *g;
            idx = tm.iter().position(|&x| x == mx).unwrap();
        }
    }
    gid as u32 * idx as u32
}

fn main() {
    println!("AoC 2018 Four");
    let records = load_records();
    let time_map = create_time_map(&records);
    println!("part1 = {}", part1(&time_map));
    println!("part2 = {}", part2(&time_map));
}
