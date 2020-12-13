/**
 * Day 8 - Handheld Halting
 */
use std::fmt;
use parse_display::{Display, FromStr};

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, FromStr, Display)]
enum Operation {
    #[display("nop {0}")]
    Nop(i32),
    #[display("acc {0}")]
    Acc(i32),
    #[display("jmp {0}")]
    Jmp(i32),
}

#[derive(Clone, Debug, Display, PartialEq)]
enum Execution {
    #[display("Loop, acc {0}")]
    Loop(i32),
    #[display("No Loop, acc {0}")]
    NoLoop(i32),
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn input_gen(input: &str) -> Vec<Operation> {
    input.lines().map(|l| { l.parse().unwrap() }).collect()
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
/** Execute the program until a loop is detected. 
 * Return the accumulator value when detected. */
fn find_loop(program: &Vec<Operation>) -> Execution {
    let mut cnt: i32 = 0; // Counter
    let mut acc: i32 = 0; // Accumulator
    let mut old_pc = 0;
    let mut pc: usize = 0; // Program Counter - index in the instruction list

    let mut exec: Vec<(i32, i32)> = Vec::new(); // Store the execution context
    for _ in program {
        exec.push( (0,0) );
    }

    // Execute program
    loop {
        cnt += 1;
        print!("{} {:?} {:?}", pc, program[pc], exec[pc]);
        if exec[pc].0 != 0 {
            println!(" | {:?} Found loop! acc = {}", exec[pc], acc);
            return Execution::Loop(acc);
        }

        exec[pc].0 = cnt;
        exec[pc].1 = acc;

        old_pc = pc;
        match program[pc] {
            Operation::Nop(_) => { 
                pc += 1;
            },
            Operation::Acc(value) => { 
                acc = acc + value;
                pc += 1;
            },
            Operation::Jmp(value) => {
                pc = (pc as i32 + value) as usize;
            },
        }
        println!(" | {:?}", exec[old_pc]);

        if pc >= program.len() {
            println!("No loop found! acc {}", acc);
            return Execution::NoLoop(acc);
        }
    }
}

/** Change the instruction pointed by PC into jmp or nop */
fn mutate(program: &Vec<Operation>, pc: usize) -> Vec<Operation> {
    let mut changed = program.clone();
    changed[pc] = match program[pc] {
        Operation::Nop(value) => Operation::Jmp(value),
        Operation::Jmp(value) => Operation::Nop(value),
        Operation::Acc(value) => Operation::Acc(value),
    };
    changed
}



// ---------------------------------------------------------------------------
#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let program = input_gen(input);
    if let Execution::Loop(value) = find_loop(&program) {
        return value
    } else {
        panic!();
    }
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let program = input_gen(input);
    for (index, op) in program.iter().enumerate() {
        let res = match op {
            Operation::Jmp(_) => {
                let changed = mutate(&program, index);
                find_loop(&changed)
            },
            Operation::Nop(_) => {
                let changed = mutate(&program, index);
                find_loop(&changed)
            },
            _ => Execution::Loop(0),
        };

        if let Execution::NoLoop(value) = res {
            return value
        }
    }
    panic!();
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_sample() {
        let program = input_gen(INPUT);
        assert_eq!(9, program.len());
        println!("{:?}", program);

        assert_eq!(Execution::Loop(5), find_loop(&program));
    }
    #[test]
    fn test_sample_mutate() {
        let program = input_gen(INPUT);
        let changed = mutate(&program, 7);
        let res = find_loop(&changed);

        assert_eq!(Execution::NoLoop(8), res);
    }
}
