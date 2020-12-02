/*
 * Split up input into lines
 */
#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<u32>
{
    input.lines().map(|l| { l.parse().unwrap() }).collect()
}

// ---------------------------------------------------------------------------

/*
 * @brief Find two entries that sum to the target
 *
 * @param input:    array of values
 * @param target:   target sum of 2 values of input
 *
 * @return product of the two values, or 0 if not found
 */
pub fn solve_2_values(input: &[u32], target: u32) -> u32
{
    //println!("Solve 2 values, target {}", target);
    // quadratic solution, bad bad bad
    for i in input.iter() {
        let x = target.saturating_sub(*i);
        if x == 0 { continue }

        for j in input.iter() {
            if j == &x {
                //println!("{} * {} = {}", i, j, i*j );
                return i*j;
            }
        }
    }
    0
}

/*
 * @brief Find three entries that sum to the target. Relies on solve_2_values
 *
 * @param input:    array of values
 * @param target:   target sum of 3 values of input
 *
 * @return product of the three values, or 0 if not found
 */
pub fn solve_3_values(input: &[u32], target: u32) -> u32
{
    for i in input.iter() {
        let x = target - i;

        let res = solve_2_values(input, x);
        if res != 0 {
            return i * res;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Entry points
#[aoc(day1, part1, naive)]
pub fn part1(input: &[u32]) -> u32
{
    solve_2_values(input, 2020)
}

#[aoc(day1, part2, naive)]
pub fn part2(input: &[u32]) -> u32
{
    solve_3_values(input, 2020)
}

// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{part1};
    use super::{part2};

    #[test]
    fn sample1() {
        let input = [1721, 979, 366, 299, 675, 1456].to_vec();
        let product = part1(&input);
        assert_eq!(product, 514579);
    }
    #[test]
    fn sample2() {
        let input = [1721, 979, 366, 299, 675, 1456].to_vec();
        let res = part2(&input);
        assert_eq!(res, 241861950);
    }
}
