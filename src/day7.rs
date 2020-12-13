/**
 * Day 7 - Handy Haversacks
 */
use std::fmt;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
/*
 * The data is stored in a directed graph of bags containing other bags:
 * "A contains B, 2 C; B contains C; C contains nothing"
 * A -> B -> C
 * A -> C
 *
 * This can be stored as an adjacency matrix:
 *  |A|B|C| 
 * -+-+-+-+-
 * A|-|1|2| 
 * B|-|-|1| 
 * C|-|-|-| 
 *
 * Rows depict the "contains" relation, with the weight being the number of bags.
 * Columns depict the "can be contained by" relation, weight doesn't have any 
 * meaning other than existence of the relation.
 */
use petgraph::Direction;
use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};
use std::collections::HashSet;

/** 3 bright white */
#[derive(Debug)]
struct Relation<'a> {
    num: u32,
    bag: &'a str,
}

/** dark orange contain 3 bright white */
#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    bags: Vec<Relation<'a>>,
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
fn bag_name(input: &str) -> &str {
    input.strip_suffix(" bags")
        .unwrap_or_else(|| input.strip_suffix(" bags.")
            .unwrap_or_else(|| input.strip_suffix(" bag")
                    .unwrap_or_else(|| input.strip_suffix(" bag.").unwrap())))
}

// "dark orange bags contain 3 bright white bags, 4 muted yellow bags."
fn parse_line<'a>(input: &'a str) -> Rule {
    let tokens: Vec<&str> = input.split(" contain ").collect();
    let bag = bag_name(tokens[0]);
    //println!("Bag: {}", bag);

    let contents: Vec<&str> = tokens[1].split(", ").collect();
    let mut rel = Vec::new();
    for item in contents {
        let entry: Vec<&str> = bag_name(item).splitn(2, " ").collect();
        let num = entry[0];
        let _bag = entry[1];
        //println!("\t{} {}", num, _bag);
        if num.starts_with("no") { continue; }
        rel.push(Relation{num:num.parse().unwrap(), bag:_bag});
    }

    Rule{name:bag, bags:rel}
}

fn all_parents<'a>(graph: &'a DiGraphMap::<&str, u32>, node: &'a str, parents: &mut HashSet<&'a str>) {
    for dad in graph.neighbors_directed(node, Direction::Incoming) {
        parents.insert(dad);
        all_parents(graph, dad, parents);
    }
}

/**
 * Return the number of bags a node must contain (recursively)
 */
fn number_of_bags<'a>(graph: &'a DiGraphMap::<&str, u32>, node: &'a str) -> u32 {
    let mut num: u32 = 0;
    println!("Computing for {}", node);
    for bag in graph.neighbors_directed(node, Direction::Outgoing) {
        println!(" - {} {}", graph.edge_weight(node, bag).unwrap(), bag);
        num = num + graph.edge_weight(node, bag).unwrap() * ( 1 + number_of_bags(graph, bag));
    }
    println!("{} bags in {}", num, node);
    num
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
//#[aoc_generator(day7)]
fn input_gen(input: &str) -> DiGraphMap::<&str, u32> {
    let num = input.lines().count();
    //println!("Input contains {} lines", num);
    let mut bags = DiGraphMap::<&str, u32>::new();
    for line in input.lines() {
        let rule = parse_line(line);
        //println!("{:?}", rule);
        for item in rule.bags.iter() {
            bags.add_edge(rule.name, item.bag, item.num);
        }
    }
    bags
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let graph = input_gen(input);
    let mut parents = HashSet::new();
    all_parents(&graph, "shiny gold", &mut parents);
    parents.len()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    let graph = input_gen(input);
    number_of_bags(&graph, "shiny gold")
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    static INPUT2: &'static str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_input_gen() {
        let graph = input_gen(INPUT);
        println!("{:?}", Dot::new(&graph));
    }
    #[test]
    fn test_sample() {
        let graph = input_gen(INPUT);
        let mut parents = HashSet::new();
        all_parents(&graph, "shiny gold", &mut parents);
        println!("{:?}", parents);
        assert_eq!(4, parents.len());
    }

    #[test]
    fn test_sample2() {
        let graph = input_gen(INPUT2);
        let num = number_of_bags(&graph, "shiny gold");
        assert_eq!(126, num);
    }
}


