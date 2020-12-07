/**
 * Day 6 - Custom Customs
 */
use std::fmt;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------
struct Group {
    /** Keep list of anyone 'yes' questions */
    any: [bool; 26],
    /** Keep list of everyone 'yes' questions */
    all: [bool; 26],
}

// ---------------------------------------------------------------------------
// Implementations
// ---------------------------------------------------------------------------
impl Group {
    fn line_to_array(line: &str) -> [bool;26] {
        let mut res: [bool; 26] = [false; 26];
        let a = 'a' as usize;
        for l in line.chars().filter(|c| c.is_ascii_lowercase()) {
            let index = l as usize - a;
            res[index] = true;
        }
        res

    }
    /** Parse "Anyone said yes" */
    fn parse(input: &String) -> Group {
        let mut res = Group{any: [false; 26], all: [true; 26]};
        for line in input.lines() {
            for (i, x) in Group::line_to_array(line).iter().enumerate() {
                if *x == true {
                    res.any[i] = true;
                } else {
                    res.all[i] = false;
                }
            }
        }
        println!("Group {} -> {} {}", input, res.nb_any(), res.nb_all());
        res
    }

    fn nb_any(&self) -> usize {
        self.any.iter().filter(|c| **c==true).count()
    }
    fn nb_all(&self) -> usize {
        self.all.iter().filter(|c| **c==true).count()
    }
}

// ---------------------------------------------------------------------------
// Input builder
// ---------------------------------------------------------------------------
fn empty_line(input: &str) -> bool {
    input.trim().is_empty()
}

#[aoc_generator(day6)]
fn input_gen(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut entry = String::new();
    for line in input.lines() {
        // empty line -> end of group
        if empty_line(line) {
            groups.push(Group::parse(&entry));
            entry.clear();
        } else {
            entry.push_str(line);
            entry.push('\n');
        }
    }
    // Don't forget last entry
    groups.push(Group::parse(&entry));

    groups
}

// ---------------------------------------------------------------------------
// Solvers
// ---------------------------------------------------------------------------
#[aoc(day6, part1)]
fn part1(groups: &[Group]) -> usize {
    groups.iter().fold(0, |acc, x| acc + x.nb_any())
}

#[aoc(day6, part2)]
fn part2(groups: &[Group]) -> usize {
    groups.iter().fold(0, |acc, x| acc + x.nb_all())
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn check_sample() {
        let groups = input_gen(SAMPLE);
        assert_eq!(groups.len(), 5);
        assert_eq!(groups[0].nb_any(), 3);
        assert_eq!(groups[1].nb_any(), 3);
        assert_eq!(groups[2].nb_any(), 3);
        assert_eq!(groups[3].nb_any(), 1);
        assert_eq!(groups[4].nb_any(), 1);

        assert_eq!(groups[0].nb_all(), 3);
        assert_eq!(groups[1].nb_all(), 0);
        assert_eq!(groups[2].nb_all(), 1);
        assert_eq!(groups[3].nb_all(), 1);
        assert_eq!(groups[4].nb_all(), 1);
    }
}
