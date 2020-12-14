// solution to
// https://adventofcode.com/2020/day/14

use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Mask(MaskInstr),
    Memset(MemsetInstr),
}
#[derive(Debug)]
struct MemsetInstr {
    addr: i64,
    val: i64,
}

#[derive(Debug)]
struct MaskInstr {
    mask: String,
    zero_mask: i64,
    one_mask: i64,
}

impl MaskInstr {
    fn new(data: &str) -> MaskInstr {
        let zero_str: String = data
            .chars()
            .map(|c| match c {
                '0' => '0',
                '1' => '1',
                'X' => '0',
                _ => '0',
            })
            .collect();
        let one_str: String = data
            .chars()
            .map(|c| match c {
                '0' => '0',
                '1' => '1',
                'X' => '1',
                _ => '1',
            })
            .collect();

        let zero_mask = i64::from_str_radix(&zero_str, 2).unwrap();
        let one_mask = i64::from_str_radix(&one_str, 2).unwrap();
        MaskInstr {
            mask: data.to_string(),
            zero_mask,
            one_mask,
        }
    }
}

type Program = Vec<Instruction>;

fn parse_1(data: &str) -> Program {
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"^mask = (.+)$").unwrap();
        static ref MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    };
    data.lines()
        .map(|l| {
            if let Some(cap) = MASK.captures(l) {
                let m = cap.get(1).unwrap().as_str().to_string();
                Instruction::Mask(MaskInstr::new(&m))
            } else if let Some(cap) = MEM.captures(l) {
                let addr = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let val = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
                Instruction::Memset(MemsetInstr { addr, val })
            } else {
                panic!("bad instruction");
            }
        })
        .collect()
}

fn part_1(data: &str) -> i64 {
    let p = parse_1(data);
    let mut mem = HashMap::new();
    let mut current_mask = &MaskInstr {
        mask: "".to_string(),
        zero_mask: 0,
        one_mask: 0,
    };
    for i in p.iter() {
        match i {
            Instruction::Mask(m) => current_mask = m,
            Instruction::Memset(ms) => {
                mem.insert(
                    ms.addr,
                    (ms.val & current_mask.one_mask) | current_mask.zero_mask,
                );
            }
        }
    }
    mem.values().sum()
}

fn resolve(mask: &str) -> Vec<String> {
    // find the index of the first X in the mask:
    if mask.contains('X') {
        let mut a = resolve(&mask.replacen("X", "0", 1));
        let mut b = resolve(&mask.replacen("X", "1", 1));
        a.append(&mut b);
        return a;
    }
    vec![mask.to_string()]
}

fn part_2(data: &str) -> i64 {
    let p = parse_1(data);
    let mut mem = HashMap::new();
    let mut current_mask = &MaskInstr {
        mask: "".to_string(),
        zero_mask: 0,
        one_mask: 0,
    };
    for i in p.iter() {
        match i {
            Instruction::Mask(m) => current_mask = m,
            Instruction::Memset(ms) => {
                let addr = format!("{:036b}", ms.addr);
                let mask = addr
                    .chars()
                    .zip(current_mask.mask.chars())
                    .map(|(ac, mc)| match (ac, mc) {
                        (ac, '0') => ac,
                        (_, '1') => '1',
                        (_, 'X') => 'X',
                        _ => {
                            panic!("bad addr/mask")
                        }
                    })
                    .collect::<String>();
                for a in resolve(&mask)
                    .iter()
                    .map(|m| i64::from_str_radix(m, 2).unwrap())
                {
                    mem.insert(a, ms.val);
                }
            }
        }
    }
    mem.values().sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    println!("part 1 = {:#?}", part_1(&contents));
    println!("part 2 = {:#?}", part_2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part1() {
        let p = parse_1(DATA);
        assert_eq!(p.len(), 4);
        assert_eq!(part_1(DATA), 165);
    }

    const DATA2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part2() {
        assert_eq!(part_2(DATA2), 208);
    }

    #[test]
    fn test_resolve() {
        assert_eq!(
            resolve("000000000000000000000000000000X1001X"),
            vec![
                "000000000000000000000000000000010010",
                "000000000000000000000000000000010011",
                "000000000000000000000000000000110010",
                "000000000000000000000000000000110011"
            ]
        )
    }
}
