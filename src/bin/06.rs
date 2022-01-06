use std::collections::HashMap;

type Seen = HashMap<Vec<u32>, u32>;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|l| l.parse().ok())
        .collect()
}

fn find_repetition(state: &mut Vec<u32>, seen: &mut Seen) -> (u32, u32) {
    let mut step = 0;

    while !seen.contains_key(state) {
        seen.insert(state.clone(), step);
        step += 1;

        let imax = state
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(i, _)| i)
            .unwrap();

        let val = state[imax];
        state[imax] = 0;

        for i in 0..val {
            let target_idx = (imax + 1 + i as usize) % state.len();
            state[target_idx] += 1;
        }
    }

    (step, step - *seen.get(state).unwrap())
}

pub fn part_one(input: &str) -> u32 {
    find_repetition(&mut parse(input), &mut HashMap::new()).0
}

pub fn part_two(input: &str) -> u32 {
    find_repetition(&mut parse(input), &mut HashMap::new()).1
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 6), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 4);
    }
}
