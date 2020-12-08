// solution to
// https://adventofcode.com/2020/day/8

use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Op {
    fn parse(data: &str) -> Op {
        let v : Vec<&str> = data.split(' ').collect();
        let param = v[1].parse::<i32>().unwrap();
        match v[0] {
            "acc" => Op::Acc(param),
            "jmp" => Op::Jmp(param),
            "nop" => Op::Nop(param),
            _ => {
                eprintln!("unknown opcode {}", v[0]);
                Op::Nop(0)
            }
        }
    }
}

#[derive(Debug)]
struct VM {
    program: Vec<Op>,
    pc: usize,
    accumulator: i32,
    executed: HashSet<usize>,
}

#[derive(Debug, PartialEq)]
enum StepResult {
    Continue,
    Failed(i32),
    Finished(i32),
}

impl VM {
    fn parse(code: &str) -> VM {
        VM {
            program: code.split('\n').map(|l| Op::parse(l)).collect(),
            pc: 0,
            accumulator: 0,
            executed: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
        self.executed.clear();
    }

    /// step the VM one instruction.
    /// if we notice a repeated instruction, return failure
    /// if we get passed the end of the program, return success
    fn step(&mut self) -> StepResult {
        if !self.executed.contains(&self.pc) {
            self.executed.insert(self.pc);
            if self.pc < self.program.len() {
                let op = &self.program[self.pc];
                eprintln!("{} {} {:?}", self.pc, self.accumulator, op);
                return match op {
                    Op::Acc(v) => {
                        self.accumulator += v;
                        self.pc += 1;
                        StepResult::Continue
                    }
                    Op::Jmp(v) => {
                        self.pc = ((self.pc as i32) + v ) as usize;
                        StepResult::Continue
                    }
                    Op::Nop(_) => {
                        self.pc += 1;
                        StepResult::Continue
                    }
                };
            } else {
                return StepResult::Finished(self.accumulator)
            }
        }
        StepResult::Failed(self.accumulator)
    }

    /// execute the program to termination
    fn run(&mut self) -> StepResult {
        loop {
            let r = self.step();
            if r != StepResult::Continue {
                return r;
            }
        }
    }

    // flip a Nop <-> Jmp
    fn mutate_one(&mut self, i:usize) {
        match self.program[i] {
            Op::Nop(v) => {
                self.program[i] = Op::Jmp(v);
            }
            Op::Jmp(v) => {
                self.program[i] = Op::Nop(v);
            },
            _ => {}
        }
    }

    /// mutate each instruction until we find a successful result
    fn mutating_run(&mut self) -> StepResult {
        for i in 0..self.program.len() {
            self.reset();
            self.mutate_one(i);
            if let StepResult::Finished(v) = self.run() {
                return StepResult::Finished(v);
            }
            self.mutate_one(i);
            eprintln!("");
        }
        StepResult::Failed(-999)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err reading the file");
    let mut vm = VM::parse(&contents);
    println!("part 1 = {:?}", vm.run());
    println!("part 2 = {:?}", vm.mutating_run());
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse() {
        let vm = VM::parse(DATA);
        assert_eq!(vm.program.len(), 9);
        assert_eq!(vm.program[0], Op::Nop(0));
        assert_eq!(vm.program[1], Op::Acc(1));
        assert_eq!(vm.program[2], Op::Jmp(4));
        assert_eq!(vm.program[3], Op::Acc(3));
        assert_eq!(vm.program[4], Op::Jmp(-3));
        assert_eq!(vm.program[5], Op::Acc(-99));
        assert_eq!(vm.program[6], Op::Acc(1));
        assert_eq!(vm.program[7], Op::Jmp(-4));
        assert_eq!(vm.program[8], Op::Acc(6));
    }

    #[test]
    fn test_part1() {
        let mut vm = VM::parse(DATA);
        assert_eq!(vm.run(), StepResult::Failed(5));
    }

    #[test]
    fn test_part2() {
        let mut vm = VM::parse(DATA);
        assert_eq!(vm.mutating_run(), StepResult::Finished(8));
    }
}
