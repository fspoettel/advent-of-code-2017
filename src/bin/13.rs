use std::cmp::max;
use std::collections::HashMap;

struct Firewall {
    layers: HashMap<usize, usize>,
    size: usize,
}

fn parse(input: &str) -> Firewall {
    let mut fw = Firewall {
        size: 0,
        layers: HashMap::new(),
    };

    input.lines().for_each(|l| {
        let mut parts = l.split(": ");
        let index = parts.next().unwrap().parse().unwrap();
        let depth = parts.next().unwrap().parse().unwrap();
        fw.layers.insert(index, depth);
        fw.size = max(fw.size, index);
    });

    fw
}

fn try_pass_firewall(fw: &Firewall, delay: usize) -> Vec<usize> {
    let mut state = vec![];

    for (pos, depth) in fw.layers.iter() {
        if (pos + delay) % ((depth - 1) * 2) == 0 {
            state.push(*pos);
        }
    }

    state
}

pub fn part_one(input: &str) -> usize {
    let fw = parse(input);
    try_pass_firewall(&fw, 0)
        .iter()
        .map(|pos| pos * fw.layers.get(pos).unwrap())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let fw = parse(input);
    let mut delay = 0;

    loop {
        if try_pass_firewall(&fw, delay).is_empty() {
            break;
        }
        delay += 1;
    }

    delay
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 13), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 24);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 10);
    }
}
