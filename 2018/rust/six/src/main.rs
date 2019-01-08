use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(s: &str) -> Point {
    let xc: Vec<i32> = s
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    Point { x: xc[0], y: xc[1] }
}

fn load_records() -> Vec<Point> {
    let r: Vec<Point> = fs::read_to_string("input.txt")
        .expect("err reading the file")
        .lines()
        .map(|l| parse(l))
        .collect();
    r
}

fn calc_distances(
    points: &Vec<Point>,
    minx: i32,
    miny: i32,
    maxx: i32,
    maxy: i32,
) -> HashMap<Point, Vec<(&Point, i32)>> {
    let mut map: HashMap<Point, Vec<(&Point, i32)>> = HashMap::new();
    for y in miny..(maxy + 1) {
        for x in minx..(maxx + 1) {
            let pds = map.entry(Point { x, y }).or_insert(vec![]);
            // given x,y; calculate the distance to every point in pt, and add it
            // to the pds Vec:
            for p in points.iter() {
                let d = (p.x - x).abs() + (p.y - y).abs();
                pds.push((p, d));
            }
            pds.sort_by_key(|e| e.1);
        }
    }
    map
}

fn largest_area(map: &HashMap<Point, Vec<(&Point, i32)>>) -> i32 {
    let mut results: HashMap<&Point, i32> = HashMap::new();
    for pds in map.values() {
        let e0 = pds[0];
        let e1 = pds[1];
        if e0.1 == e1.1 {
            // no single closest, skip:
            continue;
        } else {
            // the closest point gets one more cell added
            let c = results.entry(e0.0).or_insert(0);
            *c += 1;
        }
    }
    *results.values().max().unwrap()
}

fn sizeof_10k_region(map: &HashMap<Point, Vec<(&Point, i32)>>) -> i32 {
    let mut size = 0;
    for pds in map.values() {
        let dsum = pds.iter().fold(0, |acc, e| acc + e.1);
        if dsum < 10000 {
            size += 1;
        }
    }
    size
}

fn main() {
    println!("AoC 2018 Six");
    let points = load_records();
    let minx = points.iter().min_by_key(|p| p.x).unwrap().x;
    let maxx = points.iter().max_by_key(|p| p.x).unwrap().x;
    let miny = points.iter().min_by_key(|p| p.y).unwrap().y;
    let maxy = points.iter().max_by_key(|p| p.y).unwrap().y;
    // create the matrix holding the "map"
    let map = calc_distances(&points, minx, miny, maxx, maxy);
    println!("({},{}) - ({},{})", minx, miny, maxx, maxy);
    println!("part1 largest area = {}", largest_area(&map));
    println!(
        "part2 size of <10k distance region = {}",
        sizeof_10k_region(&map)
    );
}
