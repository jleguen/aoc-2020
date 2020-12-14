/**
 * Day 9 - Encoding Error
 */
use std::fmt;
use parse_display::{Display, FromStr};

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
/** True if num is the sum of any of the numbers in previous */
fn is_sum_of_two(previous: &[u64], num: u64) -> bool {
    println!("Is {} sum of two in {:?}?", num, previous);

    for (index, first) in previous.iter().enumerate() {
        for second in previous[index..].iter() {
            if num == first+second {
                println!("True! {} = {} + {}", num, first, second);
                return true;
            }
        }
    }
    println!("False! No corresponding sum found");
    false
}

/** Find first number which is NOT the sum of two */
fn first_invalid(list: &[u64], len: usize) -> u64 {
    for num in len .. list.len() {
        if is_sum_of_two(&list[num-len..num], list[num]) == false {
            return list[num]
        }
    }
    0
}

/** Find the contiguous numbers which sum up to num */
fn contiguous_sum(list: &[u64], num: u64) -> (usize, usize) {
    let mut sum: u64 = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;

    // Grow the slice and add to sum until we are too big.
    // Then remove elements from the front and continue until
    // we are smaller, then restart growing
    loop {
        println!("{} [{} {}]", sum, start, end);
        if sum + list[end] == num {
            println!("FOUND! [{} .. {}]", start, end);
            return (start, end);
        } else if sum + list[end] < num {
            println!("too small - add next ({})", list[end]);
            sum += list[end];
            end += 1;
        } else {
            println!("too big - remove start ({})", list[start]);
            sum -= list[start];
            start += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn input_gen(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    let list = input_gen(input);
    first_invalid(list.as_slice(), 25)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let list = input_gen(input);
    let num = first_invalid(list.as_slice(), 25);
    let (start, end) = contiguous_sum(list.as_slice(), num);
    let found = list[start..end].to_vec();
    let small = found.iter().min().unwrap();
    let big = found.iter().max().unwrap();
    small + big
}


// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_is_sum_of_two() {
        assert_eq!(true, is_sum_of_two(&[1,2,3,4], 5));
        assert_eq!(false, is_sum_of_two(&[1,2,3,4], 1));
    }

    #[test]
    fn test_sample() {
        let list = input_gen(INPUT);
        assert_eq!(127, first_invalid(list.as_slice(), 5));
    }

    #[test]
    fn test_contiguous() {
        let list = input_gen(INPUT);
        let (start, end) = contiguous_sum(list.as_slice(), 127);
        assert_eq!(2, start);
        assert_eq!(5, end);
        let found = list[start..end].to_vec();
        let small = found.iter().min().unwrap();
        let big = found.iter().max().unwrap();
        assert_eq!(62, small + big);
    }
}
