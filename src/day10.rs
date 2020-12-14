/**
 * Day 10 - Adapter Array
 */
use std::fmt;
use parse_display::{Display, FromStr};
use petgraph::dot::{Dot, Config};
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::all_simple_paths;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
// Store the adapters in a graph


// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
fn build_graph(input: &str) -> DiGraphMap<u32, u32> {
    let mut graph = DiGraphMap::<u32, u32>::new();
    let mut list = input_gen(input);
    list.push(0);
    list.sort();

    loop {
        let item = list[0];
        let remainder = list.split_off(1);
        if remainder.is_empty() {
            break;
        }
        for value in remainder.iter() {
            let diff = value - item;
            if diff > 3 { 
                break; 
            } else {
                println!("edge {} -> {} ({})", item, value, diff);
                graph.add_edge(item, *value, diff);
            }
        }
        list = remainder;
    }

    println!("{:?}", Dot::new(&graph));
    graph
}

fn check_one_three(diff: u32, ones: &mut u32, threes: &mut u32) {
    match diff {
        1 => *ones += 1,
        3 => *threes += 1,
        _ => (),
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn input_gen(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let mut list = input_gen(input);
    let mut ones = 0;
    let mut threes = 1;
    list.sort();

    // Charging output is 0
    let mut current: u32 = 0;
    for value in list.iter() {
        let diff = value - current;
        //println!("{} -> {} | {}", current, value, diff);
        check_one_three(diff, &mut ones, &mut threes);
        current = *value;
    }
    ones * threes
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "16
10
15
5
1
11
7
19
6
12
4";

    static INPUT2: &'static str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    #[test]
    fn test_sample() {
        assert_eq!(35, part1(INPUT));
        assert_eq!(220, part1(INPUT2));
    }

    #[test]
    fn test_graph() {
        let map = build_graph(INPUT);
        let graph = map.into_graph::<u32>();

        //all_simple_paths(map, 0, 19, 1, None);
    }

}
